import type { ApiResponse } from "@/models/response.model";
import api from ".";
import type { LoginRequest, LoginResponse } from "@/models/user.model";

async function login(input: LoginRequest): Promise<ApiResponse<LoginResponse>> {
    const res = await api.post('/users/login', input);

    return res.data as ApiResponse<LoginResponse>;
}

const userApi = {
    login,
}

export default userApi;