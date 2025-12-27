/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { TextSpan } from './TextSpan';
/**
 * A chunk extracted from a document.
 */
export type Chunk = {
    /**
     * Chunk content.
     */
    content: string;
    /**
     * Source document identifier.
     */
    doc_id: string;
    /**
     * Optional embedding vector.
     */
    embedding?: Array<number> | null;
    /**
     * Chunk identifier.
     */
    id: string;
    /**
     * Optional quality score for the chunk.
     */
    quality_score?: number | null;
    span: TextSpan;
    /**
     * Whether the chunk has been verified.
     */
    verified?: boolean | null;
};

