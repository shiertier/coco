/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { RerankerConfig } from './RerankerConfig';
import type { RetrievalMode } from './RetrievalMode';
import type { VectorBackendConfig } from './VectorBackendConfig';
/**
 * Query-time retrieval configuration.
 */
export type RetrievalConfig = {
    /**
     * Hybrid weight for vector vs. keyword scoring.
     */
    hybrid_alpha: number;
    reranker?: RerankerConfig | null;
    retrieval_mode: RetrievalMode;
    /**
     * Number of candidates to return.
     */
    top_k: number;
    vector_backend?: VectorBackendConfig | null;
};

