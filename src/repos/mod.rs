use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, Surreal};


pub mod base;
pub mod permission;
pub mod user;
pub mod role;
pub mod project;
pub mod environment;

#[derive(Clone)]
pub struct Combined {
    pub permission: permission::Repo,
    pub user: user::Repo,
    pub role: role::Repo,
    pub project: project::Repo,
    pub environment: environment::Repo,
}

impl Combined {
    pub fn new(db: Surreal<Client>) -> Self {
        let db = Arc::new(db);

        Self {
            permission: permission::Repo::new(Arc::clone(&db)),
            user: user::Repo::new(Arc::clone(&db)),
            role: role::Repo::new(Arc::clone(&db)),
            project: project::Repo::new(Arc::clone(&db)),
            environment: environment::Repo::new(Arc::clone(&db)),
        }
    }
}