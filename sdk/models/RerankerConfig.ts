/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
/**
 * Reranker configuration for post-retrieval scoring.
 */
export type RerankerConfig = {
    /**
     * Model name for reranking.
     */
    model_name: string;
    /**
     * Number of top candidates to rerank.
     */
    rerank_top_n: number;
};

