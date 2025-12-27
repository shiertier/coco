/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { PublicFilter } from './PublicFilter';
import type { RerankerConfig } from './RerankerConfig';
import type { RetrievalMode } from './RetrievalMode';
/**
 * Search intent describing how retrieval should run.
 */
export type PublicSearchIntent = {
    /**
     * Optional filter list (server allows `doc_id`/`chunk_id` with `eq`/`contains`).
     */
    filters: Array<PublicFilter>;
    /**
     * Hybrid weight for vector vs. keyword scoring.
     */
    hybrid_alpha: number;
    /**
     * Optional indexing configuration selection (defaults to the project's default config).
     */
    indexing_config_id?: string | null;
    /**
     * Optional query embedding (required for `vector` and `hybrid` retrieval).
     */
    query_embedding?: Array<number> | null;
    /**
     * Optional user query text (required for `fts` and `hybrid` retrieval).
     */
    query_text?: string | null;
    reranker?: RerankerConfig | null;
    retrieval_mode: RetrievalMode;
    /**
     * Number of candidates to return.
     */
    top_k: number;
};

