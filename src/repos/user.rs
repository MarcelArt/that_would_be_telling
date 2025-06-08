use std::sync::Arc;

use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{models::user::{User, UserDto}, repos::base::{ICreate, IRead, IUpdate, IDelete, IGetById}};

const USERS: &str = "users";

#[derive(Clone, IUpdate, IDelete)]
#[entity("User")]
#[dto("UserDto")]
pub struct Repo {
    db: Arc<Surreal<Client>>,
    table_name: String,
}

impl Repo {
    pub fn new(db: Arc<Surreal<Client>>) -> Self {
        Self {
            db,
            table_name: USERS.to_string(),
        }
    }
    
}

impl IRead<User> for Repo {
    async fn read(&self) -> Result<Vec<User>, crate::error::Error> {
        let query = "
            SELECT 
                *, 
                roles.{id, value, permissions} as roles_detail
            from users
        ";

        let users = self.db.query(query).await?.take(0)?;
        Ok(users)
    }
}

impl IGetById<User> for Repo { 
    async fn get_by_id(&self, id: String) -> Result<Option<User>, crate::error::Error> {
        let id = surrealdb::RecordId::from((USERS, id));
        let query = "
            SELECT 
                *, 
                roles.{id, value, permissions} as roles_detail
            from $id
        ";

        let user = self.db.query(query)
            .bind(("id", id))
            .await?
            .take(0)?;

        Ok(user)
    }
    
}

impl ICreate<User, UserDto> for Repo {
    async fn create(&self, input: UserDto) -> Result<Option<User>, crate::error::Error> {
        let query = "
            insert into users (username, email, password)
            values
            ($username, $email, crypto::argon2::generate($password))
        ";
        let user = self.db.query(query)
            .bind(("username", input.username))
            .bind(("email", input.email))
            .bind(("password", input.password))
            .await?
            .take(0)?;
        Ok(user)
    }
}

impl Repo {
    pub async fn login(&self, username: String, password: String) -> Result<Option<User>, crate::error::Error> {
        let query = "
            select 
                *, 
                roles.{id, value, permissions} as roles_detail 
            from users
            where (username = $username or email = $username)
            and crypto::argon2::compare(password, $password)
        ";
        let user = self.db.query(query)
            .bind(("username", username))
            .bind(("password", password))
            .await?
            .take(0)?;
        Ok(user)
    }
}