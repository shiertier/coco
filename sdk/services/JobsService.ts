/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { JobStatusResponse } from '../models/JobStatusResponse';
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class JobsService {
    /**
     * @param id Job identifier
     * @param xCocoOrgId Organization identifier
     * @param xCocoUserId User identifier
     * @param xCocoProjectId Project identifier
     * @returns JobStatusResponse Job status
     * @throws ApiError
     */
    public static getJob(
        id: string,
        xCocoOrgId: string,
        xCocoUserId: string,
        xCocoProjectId?: string | null,
    ): CancelablePromise<JobStatusResponse> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/v1/jobs/{id}',
            path: {
                'id': id,
            },
            headers: {
                'x-coco-org-id': xCocoOrgId,
                'x-coco-user-id': xCocoUserId,
                'x-coco-project-id': xCocoProjectId,
            },
            errors: {
                401: `Unauthorized`,
                404: `Job not found`,
            },
        });
    }
    /**
     * @param id Job identifier
     * @param xCocoOrgId Organization identifier
     * @param xCocoUserId User identifier
     * @param xCocoProjectId Project identifier
     * @returns JobStatusResponse Job event stream
     * @throws ApiError
     */
    public static jobEvents(
        id: string,
        xCocoOrgId: string,
        xCocoUserId: string,
        xCocoProjectId?: string | null,
    ): CancelablePromise<JobStatusResponse> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/v1/jobs/{id}/events',
            path: {
                'id': id,
            },
            headers: {
                'x-coco-org-id': xCocoOrgId,
                'x-coco-user-id': xCocoUserId,
                'x-coco-project-id': xCocoProjectId,
            },
        });
    }
}
