/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { HnswParams } from './HnswParams';
import type { IvfPqParams } from './IvfPqParams';
/**
 * Backend-specific index parameter overrides.
 */
export type VectorIndexParams = {
    hnsw?: HnswParams | null;
    ivf_pq?: IvfPqParams | null;
};

