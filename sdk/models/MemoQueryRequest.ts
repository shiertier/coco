/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { PublicSearchIntent } from './PublicSearchIntent';
import type { RetrievalConfig } from './RetrievalConfig';
export type MemoQueryRequest = {
    intent: PublicSearchIntent;
    retrieval_config?: RetrievalConfig | null;
    session_token: string;
};

