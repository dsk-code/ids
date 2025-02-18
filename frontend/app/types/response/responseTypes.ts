import { ResponseGetAddress } from "../streetAddress/addressTypes";
import { Class } from "../classes/classTypes";
import { PaginatedTeachersList, Teacher, TeacherId } from "../teachers/teachersTypes";

export type ResponseEntity =
    | Class
    | Class[]
    | Teacher
    | PaginatedTeachersList
    | TeacherId
    | ResponseGetAddress;
