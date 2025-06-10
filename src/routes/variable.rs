use actix_web::{middleware, web};

use crate::{handlers::variable, middlewares::auth};

pub fn setup_routes() -> actix_web::Scope {
    web::scope("/variables")
        .service(variable::read)
        .service(variable::get_by_id)
        .service(
            web::scope("")
                .wrap(middleware::from_fn(auth::protect))
                .service(variable::create)
                .service(variable::update)
                .service(variable::delete),
        )
}
