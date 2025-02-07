export interface User {
    id: string;
    name?: string;
    furigana?: string;
    class?: string;
}

export interface AuthUser {
    auth0UserName: string;
    auth0UserEmail: string;
}

export interface ResponseAuthUser {
    id: string;
    auth0UserName: string;
}