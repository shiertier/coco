/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { VectorBackendKind } from './VectorBackendKind';
/**
 * Configuration for selecting a vector backend.
 */
export type VectorBackendConfig = {
    /**
     * Optional API key or token.
     */
    api_key?: string | null;
    /**
     * Optional collection name prefix.
     */
    collection_prefix?: string | null;
    kind: VectorBackendKind;
    /**
     * Optional endpoint or connection URL.
     */
    url?: string | null;
};

