/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { IngestDocument } from './IngestDocument';
export type IngestBatchRequest = {
    activate?: boolean;
    documents: Array<IngestDocument>;
    indexing_config_id?: string | null;
};

