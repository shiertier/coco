/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
export type IngestChunk = {
    chunk_id: string;
    content: string;
    embedding: Array<number>;
    end: number;
    quality_score?: number | null;
    start: number;
    verified?: boolean | null;
};

