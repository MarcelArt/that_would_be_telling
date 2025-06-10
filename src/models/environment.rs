use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Serialize, Deserialize)]
pub struct Environment {
    pub id: RecordId,
    pub value: String,
    pub project: RecordId,
}

#[derive(Serialize, Deserialize)]
pub struct EnvironmentDto {
    pub value: String,
    pub project: RecordId,
}