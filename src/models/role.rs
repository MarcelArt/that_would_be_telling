use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::models::permission::Permission;

#[derive(Serialize, Deserialize)]
pub struct Role {
    pub id: RecordId,
    pub value: String,
    pub permissions: Vec<RecordId>,
    pub permissions_detail: Option<Vec<Permission>>,
}

#[derive(Serialize, Deserialize)]
pub struct RoleDto {
    pub value: String,
    pub permissions: Vec<RecordId>,
}

