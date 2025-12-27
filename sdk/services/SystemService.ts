/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ActivateConfigRequest } from '../models/ActivateConfigRequest';
import type { ActivateConfigResponse } from '../models/ActivateConfigResponse';
import type { HealthResponse } from '../models/HealthResponse';
import type { IndexingConfigResponse } from '../models/IndexingConfigResponse';
import type { ListConfigsResponse } from '../models/ListConfigsResponse';
import type { PruneRequest } from '../models/PruneRequest';
import type { PruneResponse } from '../models/PruneResponse';
import type { RegisterProjectRequest } from '../models/RegisterProjectRequest';
import type { RegisterProjectResponse } from '../models/RegisterProjectResponse';
import type { UpsertConfigRequest } from '../models/UpsertConfigRequest';
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class SystemService {
    /**
     * @param orgId Organization identifier
     * @param userId User identifier
     * @param projectId Project identifier
     * @returns ListConfigsResponse Indexing configs
     * @throws ApiError
     */
    public static listConfigs(
        orgId: string,
        userId?: string | null,
        projectId?: string | null,
    ): CancelablePromise<ListConfigsResponse> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/v1/sys/configs',
            query: {
                'org_id': orgId,
                'user_id': userId,
                'project_id': projectId,
            },
            errors: {
                400: `Invalid request`,
                401: `Unauthorized`,
            },
        });
    }
    /**
     * @param requestBody
     * @returns IndexingConfigResponse Indexing config
     * @throws ApiError
     */
    public static upsertConfig(
        requestBody: UpsertConfigRequest,
    ): CancelablePromise<IndexingConfigResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/v1/sys/configs',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Invalid request`,
                401: `Unauthorized`,
            },
        });
    }
    /**
     * @param requestBody
     * @returns ActivateConfigResponse Activated config
     * @throws ApiError
     */
    public static activateConfig(
        requestBody: ActivateConfigRequest,
    ): CancelablePromise<ActivateConfigResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/v1/sys/configs/activate',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Invalid request`,
                401: `Unauthorized`,
            },
        });
    }
    /**
     * @returns HealthResponse Service health
     * @throws ApiError
     */
    public static health(): CancelablePromise<HealthResponse> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/v1/sys/health',
        });
    }
    /**
     * @returns string OpenAPI document
     * @throws ApiError
     */
    public static openapiJson(): CancelablePromise<string> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/v1/sys/openapi',
        });
    }
    /**
     * @param requestBody
     * @returns PruneResponse Prune completed
     * @throws ApiError
     */
    public static pruneProject(
        requestBody: PruneRequest,
    ): CancelablePromise<PruneResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/v1/sys/prune',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Invalid request`,
                401: `Unauthorized`,
            },
        });
    }
    /**
     * @param requestBody
     * @returns RegisterProjectResponse Project registered
     * @throws ApiError
     */
    public static registerProject(
        requestBody: RegisterProjectRequest,
    ): CancelablePromise<RegisterProjectResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/v1/sys/register',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Invalid request`,
                401: `Unauthorized`,
            },
        });
    }
}
