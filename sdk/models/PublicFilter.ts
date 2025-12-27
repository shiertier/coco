/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { PublicFilterOp } from './PublicFilterOp';
/**
 * Filter constraint accepted by the public server API.
 */
export type PublicFilter = {
    /**
     * Field name to filter on (server allows `doc_id` and `chunk_id`).
     */
    field: string;
    op: PublicFilterOp;
    /**
     * String-encoded filter value.
     */
    value: string;
};

