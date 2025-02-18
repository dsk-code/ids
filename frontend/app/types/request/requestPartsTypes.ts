import { ApiPath } from "../api/apiPathTypes";
import { Payload } from "./requestPayloadTypes";

export interface GetRequestParts {
    apiPath: ApiPath;
}

export interface PostRequestParts {
    payload: Payload;
    apiPath: ApiPath;
}

export interface PutRequestParts {
    payload: Payload;
    apiPath: ApiPath;
}

export interface PatchRequestParts {
    payload: Payload;
    apiPath: ApiPath;
}

export interface DeleteRequestParts {
    apiPath: ApiPath;
}

export interface PaginationQueryParams {
    offset: number;
    limit: number;
}

