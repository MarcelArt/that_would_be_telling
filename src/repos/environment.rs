use std::sync::Arc;

use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{models::environment::{Environment, EnvironmentDto}, repos::base::{ICreate, IRead, IUpdate, IDelete, IGetById}};

const ENVIRONMENTS: &str = "environments";

#[derive(Clone, ICreate, IRead, IUpdate, IDelete, IGetById)]
#[entity("Environment")]
#[dto("EnvironmentDto")]
pub struct Repo {
    db: Arc<Surreal<Client>>,
    table_name: String,
}

impl Repo {
    pub fn new(db: Arc<Surreal<Client>>) -> Self {
        Self {
            db,
            table_name: ENVIRONMENTS.to_string(),
        }
    }
    
}