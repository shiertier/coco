//! LanceDB-backed vector storage for local mode.

#[cfg(feature = "local-storage")]
mod enabled {
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};
    use std::sync::Arc;

    use arrow_array::{
        Array, ArrayRef, FixedSizeListArray, Float32Array, Float64Array, RecordBatch,
        RecordBatchIterator, StringArray, UInt32Array,
    };
    use arrow_array::types::Float32Type;
    use arrow_schema::{DataType, Field, Schema, SchemaRef};
    use coco_protocol::{
        normalize_config_id, Chunk, ChunkId, CocoError, CocoResult, Filter, FilterOp,
        RetrievalMode, SearchHit, SearchHitMeta, SearchIntent, StorageBackend, VectorIndexParams,
        VectorMetadata, VectorMetric, VectorRecord, VectorStore,
    };
    use futures::TryStreamExt;
    use lance_index::scalar::FullTextSearchQuery;
    use lancedb::connection::CreateTableMode;
    use lancedb::index::{Index, IvfPqIndexBuilder};
    use lancedb::query::{ExecutableQuery, QueryBase, Select};
    use lancedb::table::NewColumnTransform;
    use lancedb::{connect, DistanceType, Table};
    use tokio::sync::RwLock;

    const CHUNK_TABLE: &str = "chunks";
    const DEFAULT_CONFIG_ID: &str = "default";
    const COL_CHUNK_ID: &str = "chunk_id";
    const COL_CONFIG_ID: &str = "config_id";
    const COL_VERSION_ID: &str = "version_id";
    const COL_DOC_ID: &str = "doc_id";
    const COL_CONTENT: &str = "content";
    const COL_START: &str = "start";
    const COL_END: &str = "end";
    const COL_EMBEDDING: &str = "embedding";
    const COL_DISTANCE: &str = "_distance";
    const COL_SCORE: &str = "_score";

    /// Distance metric for vector search.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum LanceDistance {
        /// L2 distance.
        L2,
        /// Cosine distance.
        Cosine,
        /// Dot product distance.
        Dot,
    }

    impl From<LanceDistance> for DistanceType {
        fn from(value: LanceDistance) -> Self {
            match value {
                LanceDistance::L2 => DistanceType::L2,
                LanceDistance::Cosine => DistanceType::Cosine,
                LanceDistance::Dot => DistanceType::Dot,
            }
        }
    }

    impl From<VectorMetric> for LanceDistance {
        fn from(value: VectorMetric) -> Self {
            match value {
                VectorMetric::Cosine => LanceDistance::Cosine,
                VectorMetric::Dot => LanceDistance::Dot,
                VectorMetric::L2 => LanceDistance::L2,
            }
        }
    }

    /// IVF-PQ vector index configuration.
    #[derive(Debug, Clone)]
    pub struct LanceIvfPqConfig {
        /// Distance metric for the index.
        pub distance: LanceDistance,
        /// Number of IVF partitions.
        pub num_partitions: Option<u32>,
        /// Number of PQ sub-vectors.
        pub num_sub_vectors: Option<u32>,
        /// Sample rate for IVF training.
        pub sample_rate: Option<u32>,
        /// Max iterations for IVF training.
        pub max_iterations: Option<u32>,
    }

    impl Default for LanceIvfPqConfig {
        fn default() -> Self {
            Self {
                distance: LanceDistance::L2,
                num_partitions: None,
                num_sub_vectors: None,
                sample_rate: None,
                max_iterations: None,
            }
        }
    }

    /// Vector index selection for LanceDB.
    #[derive(Debug, Clone)]
    pub enum LanceVectorIndex {
        /// Let LanceDB pick the index type.
        Auto,
        /// Use IVF-PQ with custom settings.
        IvfPq(LanceIvfPqConfig),
    }

    /// LanceDB vector storage backend.
    #[derive(Debug, Clone)]
    pub struct LanceBackend {
        table: Table,
        dimensions: usize,
        config_id: String,
        version_id: String,
        distance: LanceDistance,
    }

    /// Shared LanceDB manager for per-config tables.
    #[derive(Debug, Clone)]
    pub struct LanceStore {
        root: PathBuf,
        dimensions: usize,
        backends: Arc<RwLock<HashMap<(String, String), LanceBackend>>>,
    }

    impl LanceStore {
        /// Creates a new LanceDB manager rooted at the provided path.
        pub fn new(root: PathBuf, dimensions: usize) -> Self {
            Self {
                root,
                dimensions,
                backends: Arc::new(RwLock::new(HashMap::new())),
            }
        }

        /// Returns a cached backend for a specific config id, creating it when absent.
        pub async fn backend_for_config(
            &self,
            config_id: &str,
            version_id: &str,
            metric: VectorMetric,
            index_params: Option<&VectorIndexParams>,
        ) -> CocoResult<LanceBackend> {
            let config_id = ensure_canonical_config_id(config_id)?;
            let version_id = version_id.trim();
            if version_id.is_empty() {
                return Err(CocoError::user("version_id must not be empty"));
            }
            let key = (config_id.clone(), version_id.to_string());
            if let Some(existing) = self.backends.read().await.get(&key) {
                return Ok(existing.clone());
            }

            let mut backend = LanceBackend::open_with_config(
                &self.root,
                self.dimensions,
                &config_id,
                version_id,
            )
            .await?;
            backend
                .create_vector_index_with_config(metric, index_params)
                .await?;

            let mut guard = self.backends.write().await;
            if let Some(existing) = guard.get(&key) {
                return Ok(existing.clone());
            }
            guard.insert(key, backend.clone());
            Ok(backend)
        }

        /// Removes a cached backend for the given config id.
        pub async fn invalidate_config(&self, config_id: &str) -> CocoResult<()> {
            let config_id = ensure_canonical_config_id(config_id)?;
            self.backends
                .write()
                .await
                .retain(|key, _| key.0.as_str() != config_id);
            Ok(())
        }
    }

    impl LanceBackend {
        /// Opens (or creates) the default LanceDB table at the given path.
        pub async fn open(
            path: &Path,
            dimensions: usize,
            version_id: &str,
        ) -> CocoResult<Self> {
            Self::open_with_config(path, dimensions, DEFAULT_CONFIG_ID, version_id).await
        }

        /// Opens (or creates) the LanceDB table for the given config identifier.
        pub async fn open_with_config(
            path: &Path,
            dimensions: usize,
            config_id: &str,
            version_id: &str,
        ) -> CocoResult<Self> {
            if dimensions == 0 {
                return Err(CocoError::user("embedding dimensions must be greater than zero"));
            }
            let config_id = normalize_config_id(config_id)?;
            if version_id.trim().is_empty() {
                return Err(CocoError::user("version_id must not be empty"));
            }

            let path_str = path
                .to_str()
                .ok_or_else(|| CocoError::user("invalid lancedb path"))?;
            let connection = connect(path_str)
                .execute()
                .await
                .map_err(map_storage_err)?;

            let table_name = table_name_for_config(&config_id);
            let schema = chunk_schema(dimensions);
            let table = connection
                .create_empty_table(&table_name, schema)
                .mode(CreateTableMode::exist_ok(|builder| builder))
                .execute()
                .await
                .map_err(map_storage_err)?;

            ensure_config_id_column(&table, &config_id).await?;
            ensure_version_id_column(&table).await?;
            validate_schema(&table, dimensions).await?;

            Ok(Self {
                table,
                dimensions,
                config_id,
                version_id: version_id.to_string(),
                distance: LanceDistance::L2,
            })
        }

        /// Builds or refreshes the vector index.
        pub async fn create_vector_index(&self, index: LanceVectorIndex) -> CocoResult<()> {
            let index = match index {
                LanceVectorIndex::Auto => Index::Auto,
                LanceVectorIndex::IvfPq(config) => Index::IvfPq(build_ivf_pq(config)),
            };
            self.table
                .create_index(&[COL_EMBEDDING], index)
                .execute()
                .await
                .map_err(map_storage_err)
        }

        /// Builds or refreshes the vector index from protocol configuration.
        pub async fn create_vector_index_with_config(
            &mut self,
            metric: VectorMetric,
            params: Option<&VectorIndexParams>,
        ) -> CocoResult<()> {
            let (distance, index) = vector_index_from_params(metric, params)?;
            self.distance = distance;
            self.create_vector_index(index).await
        }

        /// Builds or refreshes the full-text index on content.
        pub async fn create_fts_index(&self) -> CocoResult<()> {
            self.table
                .create_index(&[COL_CONTENT], Index::FTS(Default::default()))
                .execute()
                .await
                .map_err(map_storage_err)
        }

        /// Deletes all records for the current version.
        pub async fn delete_by_version(&self) -> CocoResult<()> {
            let predicate = format!(
                "{COL_VERSION_ID} = '{}'",
                escape_literal(&self.version_id)
            );
            self.table
                .delete(&predicate)
                .await
                .map_err(map_storage_err)?;
            Ok(())
        }
    }

    /// Query executor that maps SearchIntent into LanceDB queries.
    #[derive(Clone)]
    pub struct LanceExecutor {
        table: Table,
        dimensions: usize,
        config_id: String,
        version_id: String,
        distance: LanceDistance,
    }

    impl LanceExecutor {
        /// Creates a new executor from an existing backend.
        pub fn new(backend: &LanceBackend) -> Self {
            Self {
                table: backend.table.clone(),
                dimensions: backend.dimensions,
                config_id: backend.config_id.clone(),
                version_id: backend.version_id.clone(),
                distance: backend.distance,
            }
        }

        /// Executes the search intent using LanceDB query APIs.
        pub async fn search(&self, intent: SearchIntent) -> CocoResult<Vec<SearchHit>> {
            let table = self.table.clone();
            let dimensions = self.dimensions;
            let top_k = intent.top_k().get() as usize;
            let config_id = resolve_config_id(&intent, &self.config_id)?;
            let filters = build_filter(intent.filters(), config_id, &self.version_id)?;
            match intent.retrieval_mode() {
                RetrievalMode::Vector => {
                    let embedding = intent
                        .query_embedding()
                        .ok_or_else(|| {
                            CocoError::user("query embedding required for vector search")
                        })?;
                    let mut query = table
                        .vector_search(embedding)
                        .map_err(map_storage_err)?;
                    query = query
                        .select(Select::columns(&[
                            COL_CHUNK_ID,
                            COL_DOC_ID,
                            COL_CONTENT,
                            COL_START,
                            COL_END,
                            COL_DISTANCE,
                        ]))
                        .limit(top_k)
                        .distance_type(self.distance.into());
                    if let Some(filter) = filters {
                        query = query.only_if(filter);
                    }
                    let stream = query.execute().await.map_err(map_storage_err)?;
                    let batches: Vec<RecordBatch> =
                        stream.try_collect().await.map_err(map_storage_err)?;
                    collect_vector_results(&batches, dimensions)
                }
                RetrievalMode::Fts => {
                    let query_text = intent.query_text().ok_or_else(|| {
                        CocoError::user("query text required for fts search")
                    })?;
                    let mut query = table.query().full_text_search(
                        FullTextSearchQuery::new(query_text.to_string())
                            .limit(Some(top_k as i64)),
                    );
                    if let Some(filter) = filters {
                        query = query.only_if(filter);
                    }
                    query = query.select(Select::columns(&[
                        COL_CHUNK_ID,
                        COL_DOC_ID,
                        COL_CONTENT,
                        COL_START,
                        COL_END,
                        COL_SCORE,
                    ]));
                    let stream = query.execute().await.map_err(map_storage_err)?;
                    let batches: Vec<RecordBatch> =
                        stream.try_collect().await.map_err(map_storage_err)?;
                    collect_fts_results(&batches)
                }
                RetrievalMode::Hybrid => {
                    let embedding = intent
                        .query_embedding()
                        .ok_or_else(|| {
                            CocoError::user("query embedding required for hybrid search")
                        })?;
                    let query_text = intent
                        .query_text()
                        .ok_or_else(|| CocoError::user("query text required for hybrid search"))?;

                    let mut vector_query = table
                        .vector_search(embedding)
                        .map_err(map_storage_err)?
                        .select(Select::columns(&[
                            COL_CHUNK_ID,
                            COL_DOC_ID,
                            COL_CONTENT,
                            COL_START,
                            COL_END,
                            COL_DISTANCE,
                        ]))
                        .limit(top_k)
                        .distance_type(self.distance.into());
                    if let Some(filter) = filters.clone() {
                        vector_query = vector_query.only_if(filter);
                    }
                    let vector_stream = vector_query.execute().await.map_err(map_storage_err)?;
                    let vector_batches: Vec<RecordBatch> =
                        vector_stream.try_collect().await.map_err(map_storage_err)?;
                    let vector_results = collect_vector_results(&vector_batches, dimensions)?;

                    let mut fts_query = table.query().full_text_search(
                        FullTextSearchQuery::new(query_text.to_string())
                            .limit(Some(top_k as i64)),
                    );
                    if let Some(filter) = filters {
                        fts_query = fts_query.only_if(filter);
                    }
                    fts_query = fts_query.select(Select::columns(&[
                        COL_CHUNK_ID,
                        COL_DOC_ID,
                        COL_CONTENT,
                        COL_START,
                        COL_END,
                        COL_SCORE,
                    ]));
                    let fts_stream = fts_query.execute().await.map_err(map_storage_err)?;
                    let fts_batches: Vec<RecordBatch> =
                        fts_stream.try_collect().await.map_err(map_storage_err)?;
                    let fts_results = collect_fts_results(&fts_batches)?;

                    Ok(merge_hybrid(
                        vector_results,
                        fts_results,
                        intent.hybrid_alpha(),
                        top_k,
                    ))
                }
            }
        }
    }

    impl StorageBackend for LanceBackend {
        fn upsert_chunks(&self, chunks: Vec<Chunk>) -> impl std::future::Future<Output = CocoResult<()>> + Send {
            let table = self.table.clone();
            let dimensions = self.dimensions;
            let config_id = self.config_id.clone();
            let version_id = self.version_id.clone();
            async move {
                if chunks.is_empty() {
                    return Ok(());
                }

                let batch = chunks_to_batch(&chunks, dimensions, &config_id, &version_id)?;
                let schema = batch.schema();
                let reader = RecordBatchIterator::new(vec![Ok(batch)].into_iter(), schema);
                let mut merge = table.merge_insert(&[COL_CHUNK_ID]);
                merge
                    .when_matched_update_all(None)
                    .when_not_matched_insert_all();
                merge
                    .execute(Box::new(reader))
                    .await
                    .map_err(map_storage_err)?;
                Ok(())
            }
        }

        fn search_similar(
            &self,
            intent: SearchIntent,
        ) -> impl std::future::Future<Output = CocoResult<Vec<SearchHit>>> + Send {
            let executor = LanceExecutor::new(self);
            async move { executor.search(intent).await }
        }

        fn delete_by_doc(
            &self,
            doc_id: coco_protocol::DocumentId,
        ) -> impl std::future::Future<Output = CocoResult<()>> + Send {
            let table = self.table.clone();
            let version_id = self.version_id.clone();
            async move {
                let predicate = format!(
                    "{COL_DOC_ID} = '{}' AND {COL_VERSION_ID} = '{}'",
                    escape_literal(&doc_id),
                    escape_literal(&version_id)
                );
                table
                    .delete(&predicate)
                    .await
                    .map_err(map_storage_err)?;
                Ok(())
            }
        }

        fn get_chunk(
            &self,
            chunk_id: coco_protocol::ChunkId,
        ) -> impl std::future::Future<Output = CocoResult<Option<Chunk>>> + Send {
            let table = self.table.clone();
            let version_id = self.version_id.clone();
            async move {
                let predicate = format!(
                    "{COL_CHUNK_ID} = '{}' AND {COL_VERSION_ID} = '{}'",
                    escape_literal(&chunk_id),
                    escape_literal(&version_id)
                );
                let stream = table
                    .query()
                    .only_if(predicate)
                    .limit(1)
                    .select(Select::columns(&[
                        COL_CHUNK_ID,
                        COL_DOC_ID,
                        COL_CONTENT,
                        COL_START,
                        COL_END,
                    ]))
                    .execute()
                    .await
                    .map_err(map_storage_err)?;
                let batches: Vec<RecordBatch> =
                    stream.try_collect().await.map_err(map_storage_err)?;
                let mut results = collect_chunks(&batches)?;
                Ok(results.pop().map(|chunk| Chunk {
                    embedding: None,
                    ..chunk
                }))
            }
        }
    }

    impl VectorStore for LanceBackend {
        fn upsert_vectors(
            &self,
            records: Vec<VectorRecord>,
        ) -> impl std::future::Future<Output = CocoResult<()>> + Send {
            let backend = self.clone();
            async move {
                for record in &records {
                    check_record_config_id(&record.metadata.config_id, &backend.config_id)?;
                }
                let chunks: Vec<Chunk> = records
                    .into_iter()
                    .map(|record| Chunk {
                        id: record.chunk_id,
                        doc_id: record.metadata.doc_id,
                        content: record.metadata.content,
                        embedding: Some(record.embedding),
                        span: record.metadata.span,
                        quality_score: None,
                        verified: None,
                    })
                    .collect();
                backend.upsert_chunks(chunks).await
            }
        }

        fn search_vectors(
            &self,
            intent: SearchIntent,
        ) -> impl std::future::Future<Output = CocoResult<Vec<SearchHit>>> + Send {
            self.search_similar(intent)
        }

        fn delete_vectors_by_doc(
            &self,
            doc_id: coco_protocol::DocumentId,
        ) -> impl std::future::Future<Output = CocoResult<()>> + Send {
            self.delete_by_doc(doc_id)
        }

        fn get_vector(
            &self,
            chunk_id: coco_protocol::ChunkId,
        ) -> impl std::future::Future<Output = CocoResult<Option<VectorRecord>>> + Send {
            let table = self.table.clone();
            let config_id = self.config_id.clone();
            let version_id = self.version_id.clone();
            async move {
                let predicate = format!(
                    "{COL_CHUNK_ID} = '{}' AND {COL_VERSION_ID} = '{}'",
                    escape_literal(&chunk_id),
                    escape_literal(&version_id)
                );
                let stream = table
                    .query()
                    .only_if(predicate)
                    .limit(1)
                    .select(Select::columns(&[
                        COL_CHUNK_ID,
                        COL_DOC_ID,
                        COL_CONTENT,
                        COL_START,
                        COL_END,
                        COL_EMBEDDING,
                    ]))
                    .execute()
                    .await
                    .map_err(map_storage_err)?;
                let batches: Vec<RecordBatch> =
                    stream.try_collect().await.map_err(map_storage_err)?;
                let mut chunks = Vec::new();
                let mut embeddings = Vec::new();
                for batch in &batches {
                    chunks.extend(collect_chunks(batch)?);
                    embeddings.extend(collect_embeddings(batch)?);
                }
                if chunks.is_empty() {
                    return Ok(None);
                }
                if chunks.len() != embeddings.len() {
                    return Err(CocoError::storage("embedding column mismatch"));
                }
                let chunk = chunks.remove(0);
                let embedding = embeddings.remove(0);
                Ok(Some(VectorRecord {
                    chunk_id: chunk.id,
                    embedding,
                    metadata: VectorMetadata {
                        config_id: Some(config_id),
                        doc_id: chunk.doc_id,
                        content: chunk.content,
                        span: chunk.span,
                    },
                }))
            }
        }
    }

    fn chunk_schema(dimensions: usize) -> SchemaRef {
        let embedding_field =
            Field::new("item", DataType::Float32, true);
        let embedding = DataType::FixedSizeList(Arc::new(embedding_field), dimensions as i32);
        Arc::new(Schema::new(vec![
            Field::new(COL_CHUNK_ID, DataType::Utf8, false),
            Field::new(COL_CONFIG_ID, DataType::Utf8, false),
            Field::new(COL_VERSION_ID, DataType::Utf8, false),
            Field::new(COL_DOC_ID, DataType::Utf8, false),
            Field::new(COL_CONTENT, DataType::Utf8, false),
            Field::new(COL_START, DataType::UInt32, false),
            Field::new(COL_END, DataType::UInt32, false),
            Field::new(COL_EMBEDDING, embedding, true),
        ]))
    }

    async fn validate_schema(table: &Table, dimensions: usize) -> CocoResult<()> {
        let schema = table.schema().await.map_err(map_storage_err)?;
        let field = schema.field_with_name(COL_CONFIG_ID).map_err(|err| {
            CocoError::storage(format!("config_id column missing: {err}"))
        })?;
        if !matches!(field.data_type(), DataType::Utf8 | DataType::LargeUtf8) {
            return Err(CocoError::storage(format!(
                "config_id column has unexpected type: {:?}",
                field.data_type()
            )));
        }
        let field = schema.field_with_name(COL_VERSION_ID).map_err(|err| {
            CocoError::storage(format!("version_id column missing: {err}"))
        })?;
        if !matches!(field.data_type(), DataType::Utf8 | DataType::LargeUtf8) {
            return Err(CocoError::storage(format!(
                "version_id column has unexpected type: {:?}",
                field.data_type()
            )));
        }
        let field = schema.field_with_name(COL_EMBEDDING).map_err(|err| {
            CocoError::storage(format!("embedding column missing: {err}"))
        })?;
        match field.data_type() {
            DataType::FixedSizeList(_, length) if *length as usize == dimensions => Ok(()),
            DataType::FixedSizeList(_, length) => Err(CocoError::storage(format!(
                "embedding dimension mismatch: expected {dimensions}, got {length}"
            ))),
            other => Err(CocoError::storage(format!(
                "embedding column has unexpected type: {other:?}"
            ))),
        }
    }

    async fn ensure_config_id_column(table: &Table, config_id: &str) -> CocoResult<()> {
        let schema = table.schema().await.map_err(map_storage_err)?;
        if schema.field_with_name(COL_CONFIG_ID).is_ok() {
            return Ok(());
        }
        let expression = format!("'{}'", escape_literal(config_id));
        table
            .add_columns(
                NewColumnTransform::SqlExpressions(vec![(
                    COL_CONFIG_ID.to_string(),
                    expression,
                )]),
                None,
            )
            .await
            .map_err(map_storage_err)?;
        Ok(())
    }

    async fn ensure_version_id_column(table: &Table) -> CocoResult<()> {
        let schema = table.schema().await.map_err(map_storage_err)?;
        if schema.field_with_name(COL_VERSION_ID).is_ok() {
            return Ok(());
        }
        table
            .add_columns(
                NewColumnTransform::SqlExpressions(vec![(
                    COL_VERSION_ID.to_string(),
                    "''".to_string(),
                )]),
                None,
            )
            .await
            .map_err(map_storage_err)?;
        Ok(())
    }

    fn chunks_to_batch(
        chunks: &[Chunk],
        dimensions: usize,
        config_id: &str,
        version_id: &str,
    ) -> CocoResult<RecordBatch> {
        let mut ids = Vec::with_capacity(chunks.len());
        let mut config_ids = Vec::with_capacity(chunks.len());
        let mut version_ids = Vec::with_capacity(chunks.len());
        let mut doc_ids = Vec::with_capacity(chunks.len());
        let mut contents = Vec::with_capacity(chunks.len());
        let mut starts = Vec::with_capacity(chunks.len());
        let mut ends = Vec::with_capacity(chunks.len());
        let mut embeddings = Vec::with_capacity(chunks.len());

        for chunk in chunks {
            let embedding = chunk
                .embedding
                .as_ref()
                .ok_or_else(|| CocoError::user("chunk embedding missing"))?;
            if embedding.len() != dimensions {
                return Err(CocoError::user("chunk embedding dimension mismatch"));
            }
            ids.push(chunk.id.as_str());
            config_ids.push(config_id);
            version_ids.push(version_id);
            doc_ids.push(chunk.doc_id.as_str());
            contents.push(chunk.content.as_str());
            starts.push(u32::try_from(chunk.span.start).map_err(|_| {
                CocoError::user("chunk start offset exceeds u32 range")
            })?);
            ends.push(u32::try_from(chunk.span.end).map_err(|_| {
                CocoError::user("chunk end offset exceeds u32 range")
            })?);

            let mut values = Vec::with_capacity(dimensions);
            for value in embedding {
                values.push(Some(*value));
            }
            embeddings.push(Some(values));
        }

        let schema = chunk_schema(dimensions);
        let batch = RecordBatch::try_new(
            schema,
            vec![
                Arc::new(StringArray::from(ids)),
                Arc::new(StringArray::from(config_ids)),
                Arc::new(StringArray::from(version_ids)),
                Arc::new(StringArray::from(doc_ids)),
                Arc::new(StringArray::from(contents)),
                Arc::new(UInt32Array::from(starts)),
                Arc::new(UInt32Array::from(ends)),
                Arc::new(FixedSizeListArray::from_iter_primitive::<Float32Type, _, _>(
                    embeddings,
                    dimensions as i32,
                )),
            ],
        )
        .map_err(|err| CocoError::storage(format!("failed to build batch: {err}")))?;
        Ok(batch)
    }

    fn collect_vector_results(
        batches: &[RecordBatch],
        _dimensions: usize,
    ) -> CocoResult<Vec<SearchHit>> {
        let mut results = Vec::new();
        for batch in batches {
            let chunks = collect_chunks(batch)?;
            let scores = collect_scores(batch, ScoreColumn::Distance)?;
            for (chunk, score) in chunks.into_iter().zip(scores) {
                results.push(SearchHit {
                    meta: SearchHitMeta {
                        score,
                        quality: chunk.quality_score,
                        verified: chunk.verified,
                    },
                    chunk,
                });
            }
        }
        Ok(results)
    }

    fn collect_fts_results(batches: &[RecordBatch]) -> CocoResult<Vec<SearchHit>> {
        let mut results = Vec::new();
        for batch in batches {
            let chunks = collect_chunks(batch)?;
            let scores = collect_scores(batch, ScoreColumn::Score)?;
            for (chunk, score) in chunks.into_iter().zip(scores) {
                results.push(SearchHit {
                    meta: SearchHitMeta {
                        score,
                        quality: chunk.quality_score,
                        verified: chunk.verified,
                    },
                    chunk,
                });
            }
        }
        Ok(results)
    }

    fn collect_chunks(batch: &RecordBatch) -> CocoResult<Vec<Chunk>> {
        let ids = downcast_string(batch, COL_CHUNK_ID)?;
        let doc_ids = downcast_string(batch, COL_DOC_ID)?;
        let contents = downcast_string(batch, COL_CONTENT)?;
        let starts = downcast_u32(batch, COL_START)?;
        let ends = downcast_u32(batch, COL_END)?;

        let mut chunks = Vec::with_capacity(batch.num_rows());
        for idx in 0..batch.num_rows() {
            let id = value_string(&ids, idx, COL_CHUNK_ID)?;
            let doc_id = value_string(&doc_ids, idx, COL_DOC_ID)?;
            let content = value_string(&contents, idx, COL_CONTENT)?;
            let start = value_u32(&starts, idx, COL_START)?;
            let end = value_u32(&ends, idx, COL_END)?;
            chunks.push(Chunk {
                id: id.into(),
                doc_id: doc_id.into(),
                content,
                embedding: None,
                span: coco_protocol::TextSpan {
                    start: start as usize,
                    end: end as usize,
                },
                quality_score: None,
                verified: None,
            });
        }
        Ok(chunks)
    }

    fn collect_embeddings(batch: &RecordBatch) -> CocoResult<Vec<Vec<f32>>> {
        let array = batch
            .column_by_name(COL_EMBEDDING)
            .ok_or_else(|| CocoError::storage("missing embedding column"))?;
        let list = array
            .as_any()
            .downcast_ref::<FixedSizeListArray>()
            .ok_or_else(|| CocoError::storage("invalid embedding column type"))?;
        let values = list
            .values()
            .as_any()
            .downcast_ref::<Float32Array>()
            .ok_or_else(|| CocoError::storage("invalid embedding value type"))?;
        let value_len = list.value_length() as usize;
        let mut embeddings = Vec::with_capacity(list.len());
        for row in 0..list.len() {
            if list.is_null(row) {
                return Err(CocoError::storage("embedding value missing"));
            }
            let offset = row * value_len;
            let mut embedding = Vec::with_capacity(value_len);
            for idx in 0..value_len {
                embedding.push(values.value(offset + idx));
            }
            embeddings.push(embedding);
        }
        Ok(embeddings)
    }

    enum ScoreColumn {
        Distance,
        Score,
    }

    fn collect_scores(batch: &RecordBatch, column: ScoreColumn) -> CocoResult<Vec<f32>> {
        let name = match column {
            ScoreColumn::Distance => COL_DISTANCE,
            ScoreColumn::Score => COL_SCORE,
        };
        let array = batch
            .column_by_name(name)
            .ok_or_else(|| CocoError::storage(format!("missing column {name}")))?;
        let mut scores = Vec::with_capacity(batch.num_rows());
        for idx in 0..batch.num_rows() {
            let value = read_f32(array, idx)?;
            let score = match column {
                ScoreColumn::Distance => 1.0 / (1.0 + value),
                ScoreColumn::Score => value,
            };
            scores.push(score);
        }
        Ok(scores)
    }

    fn read_f32(array: &ArrayRef, index: usize) -> CocoResult<f32> {
        if let Some(values) = array.as_any().downcast_ref::<Float32Array>() {
            if values.is_null(index) {
                return Ok(0.0);
            }
            return Ok(values.value(index));
        }
        if let Some(values) = array.as_any().downcast_ref::<Float64Array>() {
            if values.is_null(index) {
                return Ok(0.0);
            }
            let value = values.value(index);
            return Ok(value as f32);
        }
        Err(CocoError::storage("score column has unexpected type"))
    }

    fn downcast_string<'a>(
        batch: &'a RecordBatch,
        name: &str,
    ) -> CocoResult<&'a StringArray> {
        let array = batch
            .column_by_name(name)
            .ok_or_else(|| CocoError::storage(format!("missing column {name}")))?;
        array
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| CocoError::storage(format!("column {name} type mismatch")))
    }

    fn downcast_u32<'a>(
        batch: &'a RecordBatch,
        name: &str,
    ) -> CocoResult<&'a UInt32Array> {
        let array = batch
            .column_by_name(name)
            .ok_or_else(|| CocoError::storage(format!("missing column {name}")))?;
        array
            .as_any()
            .downcast_ref::<UInt32Array>()
            .ok_or_else(|| CocoError::storage(format!("column {name} type mismatch")))
    }

    fn value_string(array: &StringArray, index: usize, name: &str) -> CocoResult<String> {
        if array.is_null(index) {
            return Err(CocoError::storage(format!("column {name} is null")));
        }
        Ok(array.value(index).to_string())
    }

    fn value_u32(array: &UInt32Array, index: usize, name: &str) -> CocoResult<u32> {
        if array.is_null(index) {
            return Err(CocoError::storage(format!("column {name} is null")));
        }
        Ok(array.value(index))
    }

    fn build_filter(
        filters: &[Filter],
        config_id: &str,
        version_id: &str,
    ) -> CocoResult<Option<String>> {
        let mut parts = Vec::new();
        parts.push(format!(
            "{COL_CONFIG_ID} = '{}'",
            escape_literal(config_id)
        ));
        parts.push(format!(
            "{COL_VERSION_ID} = '{}'",
            escape_literal(version_id)
        ));
        for filter in filters {
            let field = match filter.field.as_str() {
                COL_DOC_ID | COL_CHUNK_ID | COL_CONTENT => filter.field.as_str(),
                _ => {
                    return Err(CocoError::user(format!(
                        "unsupported filter field: {}",
                        filter.field
                    )))
                }
            };
            let value = match &filter.value {
                coco_protocol::FilterValue::String(value) => value,
                _ => {
                    return Err(CocoError::user(format!(
                        "unsupported filter value for field: {}",
                        filter.field
                    )))
                }
            };
            let value = escape_literal(value);
            let expr = match filter.op {
                FilterOp::Eq => format!("{field} = '{value}'"),
                FilterOp::Contains => format!("{field} LIKE '%{value}%'"),
                _ => {
                    return Err(CocoError::user(format!(
                        "unsupported filter op: {:?}",
                        filter.op
                    )))
                }
            };
            parts.push(expr);
        }
        Ok(Some(parts.join(" AND ")))
    }

    fn resolve_config_id<'a>(
        intent: &'a SearchIntent,
        active_config_id: &'a str,
    ) -> CocoResult<&'a str> {
        match intent.indexing_config_id() {
            Some(config_id) => {
                let normalized = normalize_config_id(config_id)?;
                if normalized != active_config_id {
                    return Err(CocoError::user(
                        "indexing_config_id does not match active config",
                    ));
                }
                Ok(active_config_id)
            }
            None => Ok(active_config_id),
        }
    }

    fn check_record_config_id(
        config_id: &Option<String>,
        active_config_id: &str,
    ) -> CocoResult<()> {
        let Some(config_id) = config_id.as_deref() else {
            return Ok(());
        };
        let normalized = normalize_config_id(config_id)?;
        if normalized != active_config_id {
            return Err(CocoError::user("config_id does not match active config"));
        }
        Ok(())
    }

    fn table_name_for_config(config_id: &str) -> String {
        if config_id == DEFAULT_CONFIG_ID {
            return CHUNK_TABLE.to_string();
        }
        let mut suffix = String::with_capacity(config_id.len());
        for ch in config_id.chars() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                suffix.push(ch);
            } else {
                suffix.push('_');
            }
        }
        if suffix.is_empty() {
            CHUNK_TABLE.to_string()
        } else {
            format!("{CHUNK_TABLE}_{suffix}")
        }
    }

    fn escape_literal(value: &str) -> String {
        value.replace('\'', "''")
    }

    fn ensure_canonical_config_id(config_id: &str) -> CocoResult<String> {
        let normalized = normalize_config_id(config_id)?;
        if normalized != config_id {
            return Err(CocoError::user("config_id must be normalized"));
        }
        Ok(normalized)
    }

    fn merge_hybrid(
        vector_results: Vec<SearchHit>,
        fts_results: Vec<SearchHit>,
        alpha: f32,
        top_k: usize,
    ) -> Vec<SearchHit> {
        let mut merged: HashMap<ChunkId, SearchHit> = HashMap::new();
        let alpha = alpha.clamp(0.0, 1.0);
        let beta = 1.0 - alpha;

        for result in vector_results {
            let score = result.meta.score * alpha;
            merged.insert(
                result.chunk.id.clone(),
                SearchHit {
                    meta: SearchHitMeta {
                        score,
                        quality: result.meta.quality,
                        verified: result.meta.verified,
                    },
                    chunk: result.chunk,
                },
            );
        }

        for result in fts_results {
            let score = result.meta.score * beta;
            merged
                .entry(result.chunk.id.clone())
                .and_modify(|existing| existing.meta.score += score)
                .or_insert(SearchHit {
                    meta: SearchHitMeta {
                        score,
                        quality: result.meta.quality,
                        verified: result.meta.verified,
                    },
                    chunk: result.chunk,
                });
        }

        let mut results: Vec<SearchHit> = merged.into_values().collect();
        results.sort_by(|a, b| b.meta.score.total_cmp(&a.meta.score));
        results.truncate(top_k);
        results
    }

    fn vector_index_from_params(
        metric: VectorMetric,
        params: Option<&VectorIndexParams>,
    ) -> CocoResult<(LanceDistance, LanceVectorIndex)> {
        let distance = LanceDistance::from(metric);
        let Some(params) = params else {
            return Ok((distance, LanceVectorIndex::Auto));
        };
        if params.hnsw.is_some() {
            return Err(CocoError::user(
                "hnsw index params are not supported for lancedb",
            ));
        }
        let Some(ivf_pq) = params.ivf_pq.as_ref() else {
            return Err(CocoError::user("index_params must specify an index kind"));
        };
        let config = LanceIvfPqConfig {
            distance,
            num_partitions: ivf_pq.num_partitions,
            num_sub_vectors: ivf_pq.num_sub_vectors,
            sample_rate: ivf_pq.sample_rate,
            max_iterations: ivf_pq.max_iterations,
        };
        Ok((distance, LanceVectorIndex::IvfPq(config)))
    }

    fn build_ivf_pq(config: LanceIvfPqConfig) -> IvfPqIndexBuilder {
        let mut builder = IvfPqIndexBuilder::default().distance_type(config.distance.into());
        if let Some(num_partitions) = config.num_partitions {
            builder = builder.num_partitions(num_partitions);
        }
        if let Some(num_sub_vectors) = config.num_sub_vectors {
            builder = builder.num_sub_vectors(num_sub_vectors);
        }
        if let Some(sample_rate) = config.sample_rate {
            builder = builder.sample_rate(sample_rate);
        }
        if let Some(max_iterations) = config.max_iterations {
            builder = builder.max_iterations(max_iterations);
        }
        builder
    }

    fn map_storage_err<E: std::fmt::Display>(err: E) -> CocoError {
        CocoError::storage(format!("lancedb error: {err}"))
    }
}

#[cfg(feature = "local-storage")]
pub use enabled::{
    LanceBackend, LanceDistance, LanceExecutor, LanceIvfPqConfig, LanceStore, LanceVectorIndex,
};

#[cfg(not(feature = "local-storage"))]
mod disabled {
    use std::path::PathBuf;

    use coco_protocol::{CocoError, CocoResult, StorageBackend, VectorRecord, VectorStore};

    /// Distance metric for vector search.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum LanceDistance {
        /// L2 distance.
        L2,
        /// Cosine distance.
        Cosine,
        /// Dot product distance.
        Dot,
    }

    /// IVF-PQ vector index configuration.
    #[derive(Debug, Clone, Default)]
    pub struct LanceIvfPqConfig;

    /// Vector index selection for LanceDB.
    #[derive(Debug, Clone)]
    pub enum LanceVectorIndex {
        /// Let LanceDB pick the index type.
        Auto,
        /// Use IVF-PQ with custom settings.
        IvfPq(LanceIvfPqConfig),
    }

    /// Stub backend when local-storage feature is disabled.
    #[derive(Debug, Clone, Default)]
    pub struct LanceBackend;

    /// Stub LanceDB manager when local-storage feature is disabled.
    #[derive(Debug, Clone, Default)]
    pub struct LanceStore;

    /// Stub executor when local-storage feature is disabled.
    #[derive(Debug, Clone, Default)]
    pub struct LanceExecutor;

    impl LanceExecutor {
        /// Returns a stub executor when local-storage is disabled.
        pub fn new(_backend: &LanceBackend) -> Self {
            Self
        }

        /// Returns an error because local-storage is disabled.
        pub async fn search(
            &self,
            _intent: coco_protocol::SearchIntent,
        ) -> CocoResult<Vec<coco_protocol::SearchHit>> {
            Err(CocoError::user(
                "local-storage feature disabled for LanceBackend",
            ))
        }
    }

    impl LanceBackend {
        /// Returns an error because local-storage is disabled.
        pub async fn open(
            _path: &std::path::Path,
            _dimensions: usize,
            _version_id: &str,
        ) -> CocoResult<Self> {
            Err(CocoError::user(
                "local-storage feature disabled for LanceBackend",
            ))
        }

        /// Returns an error because local-storage is disabled.
        pub async fn open_with_config(
            _path: &std::path::Path,
            _dimensions: usize,
            _config_id: &str,
            _version_id: &str,
        ) -> CocoResult<Self> {
            Err(CocoError::user(
                "local-storage feature disabled for LanceBackend",
            ))
        }

        /// Returns an error because local-storage is disabled.
        pub async fn create_vector_index(&self, _index: LanceVectorIndex) -> CocoResult<()> {
            Err(CocoError::user(
                "local-storage feature disabled for LanceBackend",
            ))
        }

        /// Returns an error because local-storage is disabled.
        pub async fn create_vector_index_with_config(
            &mut self,
            _metric: coco_protocol::VectorMetric,
            _params: Option<&coco_protocol::VectorIndexParams>,
        ) -> CocoResult<()> {
            Err(CocoError::user(
                "local-storage feature disabled for LanceBackend",
            ))
        }

        /// Returns an error because local-storage is disabled.
        pub async fn create_fts_index(&self) -> CocoResult<()> {
            Err(CocoError::user(
                "local-storage feature disabled for LanceBackend",
            ))
        }

        /// Returns an error because local-storage is disabled.
        pub async fn delete_by_version(&self) -> CocoResult<()> {
            Err(CocoError::user(
                "local-storage feature disabled for LanceBackend",
            ))
        }
    }

    impl LanceStore {
        /// Returns a stub store when local-storage is disabled.
        pub fn new(_root: PathBuf, _dimensions: usize) -> Self {
            Self
        }

        /// Returns an error because local-storage is disabled.
        pub async fn backend_for_config(
            &self,
            _config_id: &str,
            _version_id: &str,
            _metric: coco_protocol::VectorMetric,
            _index_params: Option<&coco_protocol::VectorIndexParams>,
        ) -> CocoResult<LanceBackend> {
            Err(CocoError::user(
                "local-storage feature disabled for LanceBackend",
            ))
        }

        /// Returns an error because local-storage is disabled.
        pub async fn invalidate_config(&self, _config_id: &str) -> CocoResult<()> {
            Err(CocoError::user(
                "local-storage feature disabled for LanceBackend",
            ))
        }
    }

    impl StorageBackend for LanceBackend {
        fn upsert_chunks(
            &self,
            _chunks: Vec<coco_protocol::Chunk>,
        ) -> impl std::future::Future<Output = CocoResult<()>> + Send {
            std::future::ready(Err(CocoError::user(
                "local-storage feature disabled for LanceBackend",
            )))
        }

        fn search_similar(
            &self,
            _intent: coco_protocol::SearchIntent,
        ) -> impl std::future::Future<Output = CocoResult<Vec<coco_protocol::SearchHit>>> + Send
        {
            std::future::ready(Err(CocoError::user(
                "local-storage feature disabled for LanceBackend",
            )))
        }

        fn delete_by_doc(
            &self,
            _doc_id: coco_protocol::DocumentId,
        ) -> impl std::future::Future<Output = CocoResult<()>> + Send {
            std::future::ready(Err(CocoError::user(
                "local-storage feature disabled for LanceBackend",
            )))
        }

        fn get_chunk(
            &self,
            _chunk_id: coco_protocol::ChunkId,
        ) -> impl std::future::Future<Output = CocoResult<Option<coco_protocol::Chunk>>> + Send
        {
            std::future::ready(Err(CocoError::user(
                "local-storage feature disabled for LanceBackend",
            )))
        }
    }

    impl VectorStore for LanceBackend {
        fn upsert_vectors(
            &self,
            _records: Vec<VectorRecord>,
        ) -> impl std::future::Future<Output = CocoResult<()>> + Send {
            std::future::ready(Err(CocoError::user(
                "local-storage feature disabled for LanceBackend",
            )))
        }

        fn search_vectors(
            &self,
            _intent: coco_protocol::SearchIntent,
        ) -> impl std::future::Future<Output = CocoResult<Vec<coco_protocol::SearchHit>>> + Send
        {
            std::future::ready(Err(CocoError::user(
                "local-storage feature disabled for LanceBackend",
            )))
        }

        fn delete_vectors_by_doc(
            &self,
            _doc_id: coco_protocol::DocumentId,
        ) -> impl std::future::Future<Output = CocoResult<()>> + Send {
            std::future::ready(Err(CocoError::user(
                "local-storage feature disabled for LanceBackend",
            )))
        }

        fn get_vector(
            &self,
            _chunk_id: coco_protocol::ChunkId,
        ) -> impl std::future::Future<Output = CocoResult<Option<VectorRecord>>> + Send
        {
            std::future::ready(Err(CocoError::user(
                "local-storage feature disabled for LanceBackend",
            )))
        }
    }
}

#[cfg(not(feature = "local-storage"))]
pub use disabled::{
    LanceBackend, LanceDistance, LanceExecutor, LanceIvfPqConfig, LanceStore, LanceVectorIndex,
};
