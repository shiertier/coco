/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { MemoQueryRequest } from '../models/MemoQueryRequest';
import type { MemoQueryResponseEnvelope } from '../models/MemoQueryResponseEnvelope';
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class MemoService {
    /**
     * @param requestBody
     * @returns MemoQueryResponseEnvelope Memo query results
     * @throws ApiError
     */
    public static queryMemos(
        requestBody: MemoQueryRequest,
    ): CancelablePromise<MemoQueryResponseEnvelope> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/v1/memo/query',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Invalid request`,
                401: `Unauthorized`,
                429: `Rate limited`,
            },
        });
    }
}
