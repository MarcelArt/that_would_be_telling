use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Serialize, Deserialize)]
pub struct Permission {
    pub id: RecordId,
    pub value: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct PermissionDto {
    pub value: String,
    pub description: String,
}