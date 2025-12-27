/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CocoErrorKind } from './CocoErrorKind';
/**
 * Serializable error response used by API layers.
 */
export type ErrorResponse = {
    kind: CocoErrorKind;
    /**
     * Human-readable error message.
     */
    message: string;
};

