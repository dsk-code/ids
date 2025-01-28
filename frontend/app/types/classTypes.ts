import { User } from "./userTypes";

export interface Class {
    id: string;
    userId: string;
    className: string;
    age: number;
    createdAt: string;
    updatedAt: string;
}

export interface UserByClass {
    class_name: string;
    class_member: User[];
} 

export interface RequestPostClass {
    className: string;
    age: number;
}

export interface RequestPutClass {
    className: string;
    age: number;
}
