use std::{thread, time::Duration};

use surrealdb::{engine::remote::ws::{Client, Ws}, opt::auth::Root, Surreal};

use crate::{models::{permission::Permission, role::Role, user::UserDto}, utils::env::get_env};

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
    seeder(db.clone()).await?;
    Ok(db)
}

async fn seeder(_db: Surreal<Client>) -> Result<(), surrealdb::Error> {
    println!("Seeding database...");

    // Seed permissions
    seed_permissions(_db.clone()).await?;
    println!("Finished seeding permissions");

    // Seed roles
    seed_role(_db.clone()).await?;
    println!("Finished seeding admin role");

    // Seed admin user
    seed_user(_db.clone()).await?;
    println!("Finished seeding admin user");

    println!("Database seeding completed.");
    Ok(())
}

async fn seed_permissions(db: Surreal<Client>) -> Result<(), surrealdb::Error> {
    let query = "
        upsert permissions set
        value = 'permissions.create',
        description = 'Create permission'
        where value = 'permissions.create';

        upsert permissions set
        value = 'permissions.read',
        description = 'Read permission'
        where value = 'permissions.read';

        upsert permissions set
        value = 'permissions.update',
        description = 'Update permission'
        where value = 'permissions.update';

        upsert permissions set
        value = 'roles.delete',
        description = 'Delete role'
        where value = 'roles.delete';

        upsert permissions set
        value = 'roles.create',
        description = 'Create role'
        where value = 'roles.create';

        upsert permissions set
        value = 'roles.read',
        description = 'Read role'
        where value = 'roles.read';

        upsert permissions set
        value = 'roles.update',
        description = 'Update role'
        where value = 'roles.update';

        upsert permissions set
        value = 'roles.delete',
        description = 'Delete role'
        where value = 'roles.delete';

        upsert permissions set
        value = 'users.create',
        description = 'Create role'
        where value = 'users.create';

        upsert permissions set
        value = 'users.read',
        description = 'Read role'
        where value = 'users.read';

        upsert permissions set
        value = 'users.update',
        description = 'Update role'
        where value = 'users.update';

        upsert permissions set
        value = 'users.delete',
        description = 'Delete role'
        where value = 'users.delete';
    ";

    db.query(query).await?;
    Ok(())
}

async fn seed_role(db: Surreal<Client>) -> Result<(), surrealdb::Error> {
    let query = "
        select * from permissions
    ";

    let permissions: Vec<Permission> = db.query(query).await?.take(0)?;
    let permission_ids = permissions.iter().map(|p| p.id.clone()).collect::<Vec<_>>();

    let query = "
        upsert roles set
        value = 'Admin',
        `permissions` = $permissions
        where value = 'Admin'
    ";

    db.query(query)
        .bind(("permissions", permission_ids))
        .await?;

    Ok(())
}

async fn seed_user(db: Surreal<Client>) -> Result<(), surrealdb::Error> {
    let query = "
        select * from users where username = 'admin'
    ";
    let admin: Option<UserDto>  = db.query(query).await?.take(0)?;

    if admin.is_some() {
        println!("Admin user already exists, skipping seed.");
        return Ok(());
    }

    let query = "
        select * from roles where value = 'Admin'
    ";

    let roles: Option<Role> = db.query(query).await?.take(0)?;
    let role_ids = match roles {
        Some(r) => vec![r.id],
        None => vec![],
    };

    let query = "
        upsert users set
        username = 'admin',
        email = 'admin@yopmail.com',
        password = crypto::argon2::generate('admin'),
        roles = $roles
        where username = 'admin'
    ";

    db.query(query)
        .bind(("roles", role_ids))
        .await?;
    Ok(())
}