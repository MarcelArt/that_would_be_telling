export interface ApiResponse<T> {
    is_success: boolean;
    message: string;
    data: T;
}