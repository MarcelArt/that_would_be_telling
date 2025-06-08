use actix_web::{middleware, web};

use crate::{handlers::role, middlewares::auth};

pub fn setup_routes() -> actix_web::Scope {
    web::scope("/roles")
        .service(role::read)
        .service(role::get_by_id)
        .service(
            web::scope("")
                .wrap(middleware::from_fn(auth::protect))
                .service(role::create)
                .service(role::update)
                .service(role::delete),
        )
}
