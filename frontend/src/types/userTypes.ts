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