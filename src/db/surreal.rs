use dotenv_codegen::dotenv;
use surrealdb::{engine::remote::ws::{Client, Ws}, opt::auth::Root, Surreal};

pub async fn connect() -> Result<Surreal<Client>, Box<dyn std::error::Error>> {
    let db_host = dotenv!("DB_HOST");
    let db_user = dotenv!("DB_USER");
    let db_pass = dotenv!("DB_PASS");
    let db_ns = dotenv!("DB_NS");
    let db_db = dotenv!("DB_DB");

    let db: Surreal<Client> = Surreal::init();
    db.connect::<Ws>(db_host).await?;
    db.signin(Root {
        username: db_user,
        password: db_pass,
    }).await?;
    db.use_ns(db_ns).use_db(db_db).await?;
    println!("Connected to SurrealDB");
    Ok(db)
}