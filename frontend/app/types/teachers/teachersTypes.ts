export interface Teacher {
    id: string;
    userId: string;
    lastName: string;
    firstName: string;
    lastNameKana: string;
    firstNameKana: string;
    birthdate: string;
    phone?: string;
    mobilePhone?: string;
    email?: string;
    postCode: string;
    prefecture: string;
    city: string;
    streetAddress: string;
    building?: string;
    hireDate: string;
    leaveDate?: string;
    status: string;
    createdAt: string;
    updatedAt: string;
}

export interface PaginatedTeachersList {
    teachers: Teacher[];
    offset: number;
    limit: number;
    total: number;
}

export interface TeacherId {
    id: string;
}

export interface RequestPostTeacher {
    lastName: string;
    firstName: string;
    lastNameKana?: string;
    firstNameKana?: string;
    birthdate: string;
    phone?: string;
    mobilePhone?: string;
    email?: string;
    postCode: string;
    prefecture: string;
    city: string;
    streetAddress: string;
    building?: string;
    hireDate: string;
}

export interface RequestPutTeacher {
    lastName: string;
    firstName: string;
    lastNameKana?: string;
    firstNameKana?: string;
    birthdate: string;
    phone?: string;
    mobilePhone?: string;
    email?: string;
    postCode: string;
    prefecture: string;
    city: string;
    streetAddress: string;
    building?: string;
    hireDate: string;
}

export interface RequestPatchTeacher {
    leaveDate?: string;
    status: string;
}

export interface PaginationsWithTeachersStatus {
    offset: number;
    limit: number;
    status: string;
}