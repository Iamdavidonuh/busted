use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
use sqlx::PgPool;
use crate::routes::{get_all_course, create_course, get_course};
pub fn run_app(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let pool = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new().service(
            // prefixes all resources and routes attached to it...
            web::scope("/api")
                // ...so this handles requests for `GET /app/index.html`
                .app_data(pool.clone())
                .route("/courses", web::get().to(get_all_course))
                .route("/courses", web::post().to(create_course))
                .route("/courses/{id}", web::get().to(get_course)),
        )
    })
    .listen(listener)?
    .run();
    Ok(server)
}