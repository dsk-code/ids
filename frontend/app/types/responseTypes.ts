import { Class } from "./classTypes";
import { PaginatedTeachersList, Teacher, TeacherId } from "./teachersTypes";

export type ResponseEntity =
    | Class
    | Class[]
    | Teacher
    | PaginatedTeachersList
    | TeacherId;
