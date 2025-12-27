/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { IndexingConfig } from './IndexingConfig';
import type { PublicSearchIntent } from './PublicSearchIntent';
import type { RetrievalConfig } from './RetrievalConfig';
export type QueryRequest = {
    indexing_config?: IndexingConfig | null;
    indexing_config_id?: string | null;
    intent: PublicSearchIntent;
    retrieval_config?: RetrievalConfig | null;
};

