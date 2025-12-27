/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { QueueStatusResponse } from './QueueStatusResponse';
import type { VectorBackendStatus } from './VectorBackendStatus';
import type { WorkerStatusResponse } from './WorkerStatusResponse';
export type HealthResponse = {
    queue: QueueStatusResponse;
    service: string;
    status: string;
    vector_backend: VectorBackendStatus;
    version: string;
    worker: WorkerStatusResponse;
};

