export interface LoginRequest {
    username: string;
    password: string;
    is_remember: boolean;
}

export interface LoginResponse {
    access_token: string;
    refresh_token: string;
}