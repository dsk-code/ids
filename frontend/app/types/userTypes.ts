export interface User {
    id: string;
    name?: string;
    furigana?: string;
    class?: string;
}

export interface AuthUser {
    userName: string;
    userEmail: string;
}

export interface ResponseAuthUser {
    id: string;
    userName: string;
}