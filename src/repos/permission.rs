use std::sync::Arc;

use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{models::permission::{Permission, PermissionDto}, repos::base::{ICreate, IRead, IUpdate, IDelete, IGetById}};

// use super::base::IRepo;

const PERMISSIONS: &str = "permissions";

#[derive(Clone, ICreate, IRead, IUpdate, IDelete, IGetById)]
#[entity("Permission")]
#[dto("PermissionDto")]
pub struct Repo {
    db: Arc<Surreal<Client>>,
    table_name: String,
}

impl Repo {
    pub fn new(db: Arc<Surreal<Client>>) -> Self {
        Self {
            db,
            table_name: PERMISSIONS.to_string(),
        }
    }
    
}