/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ChunkingStrategy } from './ChunkingStrategy';
import type { EmbeddingConfig } from './EmbeddingConfig';
import type { VectorBackendConfig } from './VectorBackendConfig';
import type { VectorIndexParams } from './VectorIndexParams';
import type { VectorMetric } from './VectorMetric';
/**
 * Indexing-time configuration for a project.
 */
export type IndexingConfig = {
    chunking: ChunkingStrategy;
    /**
     * Stable identifier of this indexing strategy.
     * Note: a config_id must map to a compatible embedding dimension/metric.
     */
    config_id: string;
    embedding: EmbeddingConfig;
    index_params?: VectorIndexParams | null;
    vector_backend?: VectorBackendConfig | null;
    vector_metric: VectorMetric;
};

