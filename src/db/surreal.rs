use std::{thread, time::Duration};

use surrealdb::{engine::remote::ws::{Client, Ws}, opt::auth::Root, Surreal};

use crate::utils::env::get_env;

pub async fn connect() -> Result<Surreal<Client>, Box<dyn std::error::Error>> {
    thread::sleep(Duration::from_secs(1));

    let db_host = get_env("DB_HOST").unwrap_or_default();
    let db_user = get_env("DB_USER").unwrap_or_default();
    let db_pass = get_env("DB_PASS").unwrap_or_default();
    let db_ns = get_env("DB_NS").unwrap_or_default();
    let db_db = get_env("DB_DB").unwrap_or_default();

    println!("Connecting to SurrealDB at {}...", &db_host);

    let db: Surreal<Client> = Surreal::init();
    db.connect::<Ws>(&db_host).await?;
    db.signin(Root {
        username: db_user.as_str(),
        password: db_pass.as_str(),
    }).await?;
    db.use_ns(db_ns).use_db(db_db).await?;
    println!("Connected to SurrealDB {}", &db_host);
    Ok(db)
}