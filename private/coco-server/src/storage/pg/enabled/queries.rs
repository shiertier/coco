use coco_protocol::{
    CocoError, CocoResult, Filter, FilterOp, FilterValue, FilterValueScalar, SearchHit,
    SearchHitMeta,
};
use sea_orm::{QueryResult, Value};

use super::backend::TenantContext;
use super::helpers::{
    chunk_from_row, map_storage_err, push_value, to_i64, vector_literal, COL_CONFIG_ID,
    COL_CONTENT, COL_DISTANCE, COL_DOC_ID, COL_EMBEDDING, COL_END_LINE, COL_ID, COL_ORG_ID,
    COL_PROJECT_ID, COL_QUALITY_SCORE, COL_SCORE, COL_START_LINE, COL_USER_ID, COL_VERIFIED,
    COL_VERSION_ID, TABLE_CHUNKS,
};

#[derive(Debug, Clone, Copy)]
pub(super) enum ScoreMode {
    Distance,
    Score,
}

pub(super) fn build_vector_query(
    embedding: &[f32],
    filters: &[Filter],
    tenant: &TenantContext,
    version_id: &str,
    config_id: &str,
    top_k: usize,
    operator: &str,
) -> CocoResult<(String, Vec<Value>)> {
    let mut values = Vec::new();
    let embed_ph = push_value(&mut values, vector_literal(embedding));
    let (filter_sql, _) =
        build_filter_clause(filters, tenant, version_id, config_id, &mut values)?;
    let top_ph = push_value(&mut values, to_i64(top_k)?);
    let sql = format!(
        "SELECT {COL_ID}, {COL_DOC_ID}, {COL_CONTENT}, \
         {COL_START_LINE}, {COL_END_LINE}, {COL_QUALITY_SCORE}, {COL_VERIFIED}, \
         {COL_EMBEDDING} {operator} {embed_ph}::vector AS {COL_DISTANCE} \
         FROM {TABLE_CHUNKS} \
         WHERE {filter_sql} AND {COL_EMBEDDING} IS NOT NULL \
         ORDER BY {COL_DISTANCE} ASC \
         LIMIT {top_ph}"
    );
    Ok((sql, values))
}

pub(super) fn build_fts_query(
    query_text: &str,
    filters: &[Filter],
    tenant: &TenantContext,
    version_id: &str,
    config_id: &str,
    top_k: usize,
) -> CocoResult<(String, Vec<Value>)> {
    let mut values = Vec::new();
    let query_ph = push_value(&mut values, query_text.to_string());
    let (filter_sql, _) =
        build_filter_clause(filters, tenant, version_id, config_id, &mut values)?;
    let top_ph = push_value(&mut values, to_i64(top_k)?);
    let sql = format!(
        "SELECT {COL_ID}, {COL_DOC_ID}, {COL_CONTENT}, \
         {COL_START_LINE}, {COL_END_LINE}, {COL_QUALITY_SCORE}, {COL_VERIFIED}, \
         ts_rank_cd(to_tsvector('simple', {COL_CONTENT}), \
                    plainto_tsquery('simple', {query_ph})) AS {COL_SCORE} \
         FROM {TABLE_CHUNKS} \
         WHERE {filter_sql} \
         AND to_tsvector('simple', {COL_CONTENT}) @@ plainto_tsquery('simple', {query_ph}) \
         ORDER BY {COL_SCORE} DESC \
         LIMIT {top_ph}"
    );
    Ok((sql, values))
}

pub(super) struct HybridQueryArgs<'a> {
    pub(super) embedding: &'a [f32],
    pub(super) query_text: &'a str,
    pub(super) filters: &'a [Filter],
    pub(super) tenant: &'a TenantContext,
    pub(super) version_id: &'a str,
    pub(super) config_id: &'a str,
    pub(super) top_k: usize,
    pub(super) rrf_k: u32,
    pub(super) operator: &'a str,
}

pub(super) fn build_hybrid_query(args: HybridQueryArgs<'_>) -> CocoResult<(String, Vec<Value>)> {
    let mut values = Vec::new();
    let embed_ph = push_value(&mut values, vector_literal(args.embedding));
    let query_ph = push_value(&mut values, args.query_text.to_string());
    let (filter_sql, _) = build_filter_clause(
        args.filters,
        args.tenant,
        args.version_id,
        args.config_id,
        &mut values,
    )?;
    let candidate_k = to_i64(args.top_k.saturating_mul(2))?;
    let cand_ph = push_value(&mut values, candidate_k);
    let rrf_ph = push_value(&mut values, i64::from(args.rrf_k));
    let top_ph = push_value(&mut values, to_i64(args.top_k)?);
    let operator = args.operator;

    let sql = format!(
        "WITH vector_hits AS ( \
             SELECT {COL_ID}, {COL_DOC_ID}, {COL_CONTENT}, {COL_START_LINE}, {COL_END_LINE}, \
                    {COL_QUALITY_SCORE}, {COL_VERIFIED}, \
                    ROW_NUMBER() OVER (ORDER BY {COL_EMBEDDING} {operator} {embed_ph}::vector) \
                    AS vector_rank \
             FROM {TABLE_CHUNKS} \
             WHERE {filter_sql} AND {COL_EMBEDDING} IS NOT NULL \
             ORDER BY {COL_EMBEDDING} {operator} {embed_ph}::vector \
             LIMIT {cand_ph} \
         ), \
         fts_hits AS ( \
             SELECT {COL_ID}, {COL_DOC_ID}, {COL_CONTENT}, {COL_START_LINE}, {COL_END_LINE}, \
                    {COL_QUALITY_SCORE}, {COL_VERIFIED}, \
                    ROW_NUMBER() OVER ( \
                        ORDER BY ts_rank_cd( \
                            to_tsvector('simple', {COL_CONTENT}), \
                            plainto_tsquery('simple', {query_ph}) \
                        ) DESC \
                    ) AS fts_rank \
             FROM {TABLE_CHUNKS} \
             WHERE {filter_sql} \
             AND to_tsvector('simple', {COL_CONTENT}) @@ plainto_tsquery('simple', {query_ph}) \
             ORDER BY ts_rank_cd( \
                 to_tsvector('simple', {COL_CONTENT}), \
                 plainto_tsquery('simple', {query_ph}) \
             ) DESC \
             LIMIT {cand_ph} \
         ), \
         combined AS ( \
             SELECT \
                 COALESCE(vector_hits.{COL_ID}, fts_hits.{COL_ID}) AS {COL_ID}, \
                 COALESCE(vector_hits.{COL_DOC_ID}, fts_hits.{COL_DOC_ID}) AS {COL_DOC_ID}, \
                 COALESCE(vector_hits.{COL_CONTENT}, fts_hits.{COL_CONTENT}) AS {COL_CONTENT}, \
                 COALESCE(vector_hits.{COL_START_LINE}, fts_hits.{COL_START_LINE}) AS {COL_START_LINE}, \
                 COALESCE(vector_hits.{COL_END_LINE}, fts_hits.{COL_END_LINE}) AS {COL_END_LINE}, \
                 COALESCE(vector_hits.{COL_QUALITY_SCORE}, fts_hits.{COL_QUALITY_SCORE}) AS {COL_QUALITY_SCORE}, \
                 COALESCE(vector_hits.{COL_VERIFIED}, fts_hits.{COL_VERIFIED}) AS {COL_VERIFIED}, \
                 vector_hits.vector_rank, \
                 fts_hits.fts_rank \
             FROM vector_hits \
             FULL OUTER JOIN fts_hits ON vector_hits.{COL_ID} = fts_hits.{COL_ID} \
         ) \
         SELECT {COL_ID}, {COL_DOC_ID}, {COL_CONTENT}, {COL_START_LINE}, {COL_END_LINE}, \
                {COL_QUALITY_SCORE}, {COL_VERIFIED}, \
                (COALESCE(1.0 / ({rrf_ph} + vector_rank), 0.0) + \
                 COALESCE(1.0 / ({rrf_ph} + fts_rank), 0.0)) AS {COL_SCORE} \
         FROM combined \
         ORDER BY {COL_SCORE} DESC \
         LIMIT {top_ph}"
    );

    Ok((sql, values))
}

pub(super) fn build_filter_clause(
    filters: &[Filter],
    tenant: &TenantContext,
    version_id: &str,
    config_id: &str,
    values: &mut Vec<Value>,
) -> CocoResult<(String, usize)> {
    let mut clauses = Vec::new();
    let org_ph = push_value(values, tenant.org_id.clone());
    let user_ph = push_value(values, tenant.user_id.clone());
    let project_ph = push_value(values, tenant.project_id.clone());
    let version_ph = push_value(values, version_id.to_string());
    let config_ph = push_value(values, config_id.to_string());
    clauses.push(format!("{COL_ORG_ID} = {org_ph}"));
    clauses.push(format!("{COL_USER_ID} = {user_ph}"));
    clauses.push(format!("{COL_PROJECT_ID} = {project_ph}"));
    clauses.push(format!("{COL_VERSION_ID} = {version_ph}"));
    clauses.push(format!("{COL_CONFIG_ID} = {config_ph}"));

    for filter in filters {
        apply_filter(filter, &mut clauses, values)?;
    }

    Ok((clauses.join(" AND "), values.len()))
}

fn apply_filter(
    filter: &Filter,
    clauses: &mut Vec<String>,
    values: &mut Vec<Value>,
) -> CocoResult<()> {
    let column = filter_column(filter.field.as_str())
        .ok_or_else(|| CocoError::user("unsupported filter field"))?;
    match column.kind {
        FilterKind::Text => apply_text_filter(filter, column.name, clauses, values),
        FilterKind::Number => apply_number_filter(filter, column.name, clauses, values),
    }
}

fn apply_text_filter(
    filter: &Filter,
    column: &str,
    clauses: &mut Vec<String>,
    values: &mut Vec<Value>,
) -> CocoResult<()> {
    match filter.op {
        FilterOp::Eq | FilterOp::Neq => {
            let op = if matches!(filter.op, FilterOp::Eq) { "=" } else { "!=" };
            let value = match &filter.value {
                FilterValue::String(value) => value,
                _ => {
                    return Err(CocoError::user(
                        "text filter requires a string value",
                    ))
                }
            };
            let value_ph = push_value(values, value.clone());
            clauses.push(format!("{column} {op} {value_ph}"));
            Ok(())
        }
        FilterOp::Contains => {
            let value = match &filter.value {
                FilterValue::String(value) => value,
                _ => {
                    return Err(CocoError::user(
                        "text filter requires a string value",
                    ))
                }
            };
            let value_ph = push_value(values, format!("%{}%", value));
            clauses.push(format!("{column} ILIKE {value_ph}"));
            Ok(())
        }
        FilterOp::In => {
            let items = match &filter.value {
                FilterValue::List(values) => values
                    .iter()
                    .map(|item| match item {
                        FilterValueScalar::String(value) => Ok(value.clone()),
                        _ => Err(CocoError::user(
                            "text filter list requires string values",
                        )),
                    })
                    .collect::<CocoResult<Vec<_>>>()?,
                _ => {
                    return Err(CocoError::user(
                        "text filter requires a list value",
                    ))
                }
            };
            if items.is_empty() {
                return Err(CocoError::user("filter list must not be empty"));
            }
            let mut placeholders = Vec::with_capacity(items.len());
            for item in items {
                placeholders.push(push_value(values, item));
            }
            clauses.push(format!("{column} IN ({})", placeholders.join(", ")));
            Ok(())
        }
        _ => Err(CocoError::user("unsupported filter op for text field")),
    }
}

fn apply_number_filter(
    filter: &Filter,
    column: &str,
    clauses: &mut Vec<String>,
    values: &mut Vec<Value>,
) -> CocoResult<()> {
    match filter.op {
        FilterOp::Eq
        | FilterOp::Neq
        | FilterOp::Gt
        | FilterOp::Gte
        | FilterOp::Lt
        | FilterOp::Lte => {
            let op = match filter.op {
                FilterOp::Eq => "=",
                FilterOp::Neq => "!=",
                FilterOp::Gt => ">",
                FilterOp::Gte => ">=",
                FilterOp::Lt => "<",
                FilterOp::Lte => "<=",
                _ => unreachable!(),
            };
            let value = match &filter.value {
                FilterValue::Int(value) => *value,
                FilterValue::String(value) => value
                    .parse::<i64>()
                    .map_err(|_| CocoError::user("numeric filter requires integer value"))?,
                _ => {
                    return Err(CocoError::user(
                        "numeric filter requires integer value",
                    ))
                }
            };
            let value_ph = push_value(values, value);
            clauses.push(format!("{column} {op} {value_ph}"));
            Ok(())
        }
        FilterOp::In => {
            let items = match &filter.value {
                FilterValue::List(values) => values
                    .iter()
                    .map(|item| match item {
                        FilterValueScalar::Int(value) => Ok(*value),
                        FilterValueScalar::String(value) => value
                            .parse::<i64>()
                            .map_err(|_| CocoError::user("numeric filter requires integer value")),
                        _ => Err(CocoError::user(
                            "numeric filter list requires integer values",
                        )),
                    })
                    .collect::<CocoResult<Vec<_>>>()?,
                _ => {
                    return Err(CocoError::user(
                        "numeric filter requires list value",
                    ))
                }
            };
            if items.is_empty() {
                return Err(CocoError::user("filter list must not be empty"));
            }
            let mut placeholders = Vec::with_capacity(items.len());
            for item in items {
                placeholders.push(push_value(values, item));
            }
            clauses.push(format!("{column} IN ({})", placeholders.join(", ")));
            Ok(())
        }
        _ => Err(CocoError::user("unsupported filter op for numeric field")),
    }
}

fn filter_column(field: &str) -> Option<FilterColumn> {
    match field {
        "org_id" => Some(FilterColumn::text(COL_ORG_ID)),
        "project_id" => Some(FilterColumn::text(COL_PROJECT_ID)),
        "doc_id" => Some(FilterColumn::text(COL_DOC_ID)),
        "chunk_id" => Some(FilterColumn::text(COL_ID)),
        "version_id" => Some(FilterColumn::text(COL_VERSION_ID)),
        "start_line" => Some(FilterColumn::number(COL_START_LINE)),
        "end_line" => Some(FilterColumn::number(COL_END_LINE)),
        _ => None,
    }
}

#[derive(Clone, Copy)]
struct FilterColumn {
    name: &'static str,
    kind: FilterKind,
}

impl FilterColumn {
    fn text(name: &'static str) -> Self {
        Self {
            name,
            kind: FilterKind::Text,
        }
    }

    fn number(name: &'static str) -> Self {
        Self {
            name,
            kind: FilterKind::Number,
        }
    }
}

#[derive(Clone, Copy)]
enum FilterKind {
    Text,
    Number,
}

pub(super) fn collect_results(
    rows: Vec<QueryResult>,
    score_column: &str,
    mode: ScoreMode,
) -> CocoResult<Vec<SearchHit>> {
    let mut results = Vec::with_capacity(rows.len());
    let mut min_score = f32::INFINITY;
    let mut max_score = f32::NEG_INFINITY;
    for row in rows {
        let chunk = chunk_from_row(&row)?;
        let score = match mode {
            ScoreMode::Distance => {
                let distance: f32 = row.try_get("", score_column).map_err(map_storage_err)?;
                1.0 / (1.0 + distance)
            }
            ScoreMode::Score => {
                let score: f32 = row.try_get("", score_column).map_err(map_storage_err)?;
                score
            }
        };
        min_score = min_score.min(score);
        max_score = max_score.max(score);
        results.push(SearchHit {
            meta: SearchHitMeta {
                score,
                quality: chunk.quality_score,
                verified: chunk.verified,
            },
            chunk,
        });
    }
    normalize_scores(results, min_score, max_score)
}

fn normalize_scores(
    mut results: Vec<SearchHit>,
    min_score: f32,
    max_score: f32,
) -> CocoResult<Vec<SearchHit>> {
    if results.is_empty() {
        return Ok(results);
    }
    let needs_normalization = min_score < 0.0 || max_score > 1.0;
    if needs_normalization {
        let range = (max_score - min_score).max(f32::EPSILON);
        for hit in &mut results {
            let normalized = (hit.meta.score - min_score) / range;
            hit.meta.score = normalized.clamp(0.0, 1.0);
        }
    } else {
        for hit in &mut results {
            hit.meta.score = hit.meta.score.clamp(0.0, 1.0);
        }
    }
    Ok(results)
}
