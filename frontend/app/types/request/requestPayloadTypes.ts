import { RequestGetAddress } from "../streetAddress/addressTypes";
import { RequestPostClass, RequestPutClass } from "../classes/classTypes";
import { RequestPatchTeacher, RequestPostTeacher, RequestPutTeacher } from "../teachers/teachersTypes";

export type Payload =
    | RequestPostClass
    | RequestPutClass
    | RequestPostTeacher
    | RequestPutTeacher
    | RequestPatchTeacher
