use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::models::user::User;

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: RecordId,
    pub name: String,
    pub creator: RecordId,
    pub creator_detail: Option<User>,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectDto {
    pub name: String,
    pub creator: Option<RecordId>,
}