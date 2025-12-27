/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { IngestChunk } from './IngestChunk';
export type IngestDocument = {
    chunks: Array<IngestChunk>;
    content_hash?: string | null;
    doc_id: string;
    quality_score?: number | null;
    source_ref: string;
    title?: string | null;
    verified?: boolean | null;
};

