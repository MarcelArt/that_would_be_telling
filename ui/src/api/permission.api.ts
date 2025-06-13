import type { Permission } from "@/models/permission.model";
import api from ".";

async function read(): Promise<Permission[]> {
    const res = await api.get('/permissions');

    return res.data as Permission[];
}

const permissionApi = {
    
}