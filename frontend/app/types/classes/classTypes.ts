export interface Class {
    id: string;
    userId: string;
    className: string;
    age: number;
    createdAt: string;
    updatedAt: string;
}

export interface RequestPostClass {
    className: string;
    age: number;
}

export interface RequestPutClass {
    className: string;
    age: number;
}
