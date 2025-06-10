use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Serialize, Deserialize)]
pub struct Variable {
    pub id: RecordId,
    pub key: String,
    pub value: String,
    pub environment: RecordId,
}

#[derive(Serialize, Deserialize)]
pub struct VariableDto {
    pub key: String,
    pub value: String,
    pub environment: RecordId,
}