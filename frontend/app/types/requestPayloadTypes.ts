import { RequestPostClass, RequestPutClass } from "./classTypes";
import { RequestPatchTeacher, RequestPostTeacher, RequestPutTeacher } from "./teachersTypes";

export type Payload =
    | RequestPostClass
    | RequestPutClass
    | RequestPostTeacher
    | RequestPutTeacher
    | RequestPatchTeacher;