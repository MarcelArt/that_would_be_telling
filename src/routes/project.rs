use actix_web::{middleware, web};

use crate::{handlers::project, middlewares::auth};

pub fn setup_routes() -> actix_web::Scope {
    web::scope("/projects")
        .service(project::read)
        .service(project::get_by_id)
        .service(
            web::scope("")
                .wrap(middleware::from_fn(auth::protect))
                .service(project::create)
                .service(project::update)
                .service(project::delete),
        )
}
