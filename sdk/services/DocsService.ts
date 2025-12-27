/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { IndexRequest } from '../models/IndexRequest';
import type { IndexResponse } from '../models/IndexResponse';
import type { IngestBatchRequest } from '../models/IngestBatchRequest';
import type { IngestBatchResponse } from '../models/IngestBatchResponse';
import type { QueryRequest } from '../models/QueryRequest';
import type { QueryResponseEnvelope } from '../models/QueryResponseEnvelope';
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class DocsService {
    /**
     * @param xCocoOrgId Organization identifier
     * @param xCocoUserId User identifier
     * @param xCocoProjectId Project identifier
     * @param requestBody
     * @returns IngestBatchResponse Batch accepted
     * @throws ApiError
     */
    public static importDocuments(
        xCocoOrgId: string,
        xCocoUserId: string,
        xCocoProjectId: string,
        requestBody: IngestBatchRequest,
    ): CancelablePromise<IngestBatchResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/v1/docs/import',
            headers: {
                'x-coco-org-id': xCocoOrgId,
                'x-coco-user-id': xCocoUserId,
                'x-coco-project-id': xCocoProjectId,
            },
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Invalid request`,
                401: `Unauthorized`,
                429: `Rate limited`,
            },
        });
    }
    /**
     * @param xCocoOrgId Organization identifier
     * @param xCocoUserId User identifier
     * @param xCocoProjectId Project identifier
     * @param requestBody
     * @returns IndexResponse Reindex accepted
     * @throws ApiError
     */
    public static indexDocuments(
        xCocoOrgId: string,
        xCocoUserId: string,
        xCocoProjectId: string,
        requestBody: IndexRequest,
    ): CancelablePromise<IndexResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/v1/docs/index',
            headers: {
                'x-coco-org-id': xCocoOrgId,
                'x-coco-user-id': xCocoUserId,
                'x-coco-project-id': xCocoProjectId,
            },
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Invalid request`,
                401: `Unauthorized`,
                429: `Rate limited`,
            },
        });
    }
    /**
     * @param xCocoOrgId Organization identifier
     * @param xCocoUserId User identifier
     * @param xCocoProjectId Project identifier
     * @param requestBody
     * @returns QueryResponseEnvelope Query results
     * @throws ApiError
     */
    public static queryDocuments(
        xCocoOrgId: string,
        xCocoUserId: string,
        xCocoProjectId: string,
        requestBody: QueryRequest,
    ): CancelablePromise<QueryResponseEnvelope> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/v1/docs/query',
            headers: {
                'x-coco-org-id': xCocoOrgId,
                'x-coco-user-id': xCocoUserId,
                'x-coco-project-id': xCocoProjectId,
            },
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Invalid request`,
                401: `Unauthorized`,
                429: `Rate limited`,
            },
        });
    }
    /**
     * @param xCocoOrgId Organization identifier
     * @param xCocoUserId User identifier
     * @param xCocoProjectId Project identifier
     * @param requestBody
     * @returns IngestBatchResponse Batch accepted
     * @throws ApiError
     */
    public static ingestBatch(
        xCocoOrgId: string,
        xCocoUserId: string,
        xCocoProjectId: string,
        requestBody: IngestBatchRequest,
    ): CancelablePromise<IngestBatchResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/v1/ingest/batch',
            headers: {
                'x-coco-org-id': xCocoOrgId,
                'x-coco-user-id': xCocoUserId,
                'x-coco-project-id': xCocoProjectId,
            },
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
