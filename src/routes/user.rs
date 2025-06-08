use actix_web::{middleware, web};

use crate::{handlers::user, middlewares::auth};

pub fn setup_routes() -> actix_web::Scope {
    web::scope("/users")
        .service(user::create)
        .service(user::read)
        .service(user::login)
        .service(user::generate_new_token_pair)
        .service(user::get_by_id)
        .service(
            web::scope("path")
                .wrap(middleware::from_fn(auth::protect))
                .service(user::update)
                .service(user::delete)
        )
}