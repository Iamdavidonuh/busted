use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Course {
    id: Uuid,
    name: String,
    author: String,
    subscribed_at: chrono::DateTime<Utc>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CreateCourse {
    name: String,
    author: String,
}

pub async fn get_all_course(pool: web::Data<PgPool>) -> HttpResponse {
    let payload = sqlx::query_as!(
        Course,
        r#"SELECT * FROM courses
        "#,
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap();
    HttpResponse::Ok().json(payload)
}
pub async fn create_course(
    payload: web::Json<CreateCourse>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let response = sqlx::query_as!(
        Course,
        r#"INSERT INTO courses (id, name, author, subscribed_at)
        VALUES ($1, $2, $3, $4) returning *;
        "#,
        Uuid::new_v4(),
        payload.name,
        payload.author,
        Utc::now(),
    )
    .fetch_one(pool.get_ref())
    .await
    .unwrap();
    HttpResponse::Ok().json(response)
}

pub async fn get_course(id: web::Path<Uuid>, pool: web::Data<PgPool>) -> HttpResponse {
    let payload = sqlx::query_as!(
        Course,
        r#"SELECT * FROM courses WHERE id = $1
        "#,
        id.into_inner()
    )
    .fetch_one(pool.get_ref())
    .await
    .unwrap();
    HttpResponse::Ok().json(payload)
}
