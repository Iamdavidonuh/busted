pub mod configurations;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};

async fn get_all_course() -> impl Responder {
    HttpResponse::Ok()
}
async fn create_course() -> impl Responder {
    HttpResponse::Ok()
}

async fn get_course(id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok()
}
pub fn run_app(address: &str) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new().service(
            // prefixes all resources and routes attached to it...
            web::scope("/api")
                // ...so this handles requests for `GET /app/index.html`
                .route("/courses", web::get().to(get_all_course))
                .route("/courses", web::post().to(create_course))
                .route("/courses/{id}", web::get().to(get_course)),
        )
    })
    .bind(address)?
    .run();
    Ok(server)
}
