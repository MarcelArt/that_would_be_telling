use actix_web::{middleware, web};

use crate::{handlers::environment, middlewares::auth};

pub fn setup_routes() -> actix_web::Scope {
    web::scope("/environments")
        .service(environment::read)
        .service(environment::get_by_id)
        .service(
            web::scope("")
                .wrap(middleware::from_fn(auth::protect))
                .service(environment::create)
                .service(environment::update)
                .service(environment::delete),
        )
}
