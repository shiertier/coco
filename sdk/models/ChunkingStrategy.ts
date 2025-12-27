/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
/**
 * Chunking strategy parameters.
 */
export type ChunkingStrategy = {
    /**
     * Overlap size between adjacent chunks.
     */
    chunk_overlap: number;
    /**
     * Target chunk size in tokens or characters.
     */
    chunk_size: number;
    /**
     * Strategy name (e.g., markdown_header, fixed_token).
     */
    strategy_name: string;
};

