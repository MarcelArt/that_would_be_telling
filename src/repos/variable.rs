use std::sync::Arc;

use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{models::variable::{Variable, VariableDto}, repos::base::{ICreate, IRead, IUpdate, IDelete, IGetById}};

const VARIABLES: &str = "variables";

#[derive(Clone, ICreate, IRead, IUpdate, IDelete, IGetById)]
#[entity("Variable")]
#[dto("VariableDto")]
pub struct Repo {
    db: Arc<Surreal<Client>>,
    table_name: String,
}

impl Repo {
    pub fn new(db: Arc<Surreal<Client>>) -> Self {
        Self {
            db,
            table_name: VARIABLES.to_string(),
        }
    }
    
}