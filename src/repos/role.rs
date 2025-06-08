use crate::{
    models::role::{Role, RoleDto},
    repos::base::{ICreate, IDelete, IGetById, IRead, IUpdate},
};
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, RecordId, Surreal};

const ROLES: &str = "roles";

#[derive(Clone, ICreate, IUpdate, IDelete)]
#[entity("Role")]
#[dto("RoleDto")]
pub struct Repo {
    db: Arc<Surreal<Client>>,
    table_name: String,
}

impl Repo {
    pub fn new(db: Arc<Surreal<Client>>) -> Self {
        Self {
            db,
            table_name: ROLES.to_string(),
        }
    }
}

impl IRead<Role> for Repo {
    async fn read(&self) -> Result<Vec<Role>, crate::error::Error> {
        let query = "
            SELECT 
                *, 
                permissions.{id, value, description} as permissions_detail
            from roles
        ";

        let roles = self.db.query(query).await?.take(0)?;

        Ok(roles)
    }
}


impl IGetById<Role> for Repo { 
    async fn get_by_id(&self, id: String) -> Result<Option<Role>, crate::error::Error> {
        let id = RecordId::from((ROLES, id));
        let query = "
            SELECT 
                *, 
                permissions.{id, value, description} as permissions_detail
            from $id
        ";

        let role = self.db.query(query)
            .bind(("id", id))
            .await?
            .take(0)?;

        Ok(role)
    }
}
