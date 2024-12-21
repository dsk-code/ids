import { User } from "./userTypes";

export interface Class {
    class_id: number;
    class_name?: string;
    age?: number;
}

export interface UserByClass {
    class_name: string;
    class_member: User[];
}