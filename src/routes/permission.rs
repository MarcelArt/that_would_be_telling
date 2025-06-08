use actix_web::{middleware, web};

use crate::{handlers::permission, middlewares::auth};

pub fn setup_routes() -> actix_web::Scope {
    web::scope("/permissions")
        .service(permission::read)
        .service(permission::get_by_id)
        .service(
            web::scope("")
                .wrap(middleware::from_fn(auth::protect))
                .service(permission::create)
                .service(permission::update)
                .service(permission::delete),
        )
}
