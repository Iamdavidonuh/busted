use actix_web::{web, HttpResponse};
use sqlx::PgPool;

pub async fn get_all_course(pool : web::Data<PgPool>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
pub async fn create_course(pool : web::Data<PgPool>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn get_course(id: web::Path<u32>, pool : web::Data<PgPool>) -> HttpResponse{
    HttpResponse::Ok().finish()
}