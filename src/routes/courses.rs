use chrono::Utc;
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

#[tracing::instrument(
    name = "Getting all courses", skip(pool))]
pub async fn get_all_course(pool: web::Data<PgPool>) -> HttpResponse {
    match get_all_courses( &pool).await {
        Ok(payload) => HttpResponse::Ok().json(payload),
        Err(e) => {
            tracing::error!("An error occured when getting all courses {}", e);
            HttpResponse::BadGateway().finish()
        }
    }
}

#[tracing::instrument(
    name = "Getting all courses from database", skip(pool))]
pub async fn get_all_courses(pool : &PgPool) -> Result<Vec<Course>, sqlx::Error>{
    let payload = sqlx::query_as!(
        Course,
        r#"SELECT * FROM courses
        "#,
    )
    .fetch_all(pool)
    .await;
    payload
}

#[tracing::instrument(
    name = "Creating a new course",
    skip(payload, pool),
    fields(author = payload.author, name = payload.name)
)]
pub async fn create_course(
    payload: web::Json<CreateCourse>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match insert_course(&payload, &pool).await {
        Ok(payload) => HttpResponse::Ok().json(payload),
        Err(e) => {
            tracing::error!("An error occured creating course:  {}", e);
            HttpResponse::BadGateway().finish()
        }
    }
}


#[tracing::instrument(
    name = "Inserting a course",
    skip(data, pool),
)]
pub async fn insert_course(data : &CreateCourse, pool: &PgPool) -> Result<Course, sqlx::Error>{
    let response = sqlx::query_as!(
        Course,
        r#"INSERT INTO courses (id, name, author, subscribed_at)
        VALUES ($1, $2, $3, $4) returning *;
        "#,
        Uuid::new_v4(),
        data.name.clone(),
        data.author.clone(),
        Utc::now(),
    ).fetch_one(pool).await;
    response
}

#[tracing::instrument(
    name = "getting_course_by_id",
    skip(pool),
)]
pub async fn get_course(id: web::Path<Uuid>, pool: web::Data<PgPool>) -> HttpResponse {
    match get_course_by_id(id.into_inner(), &pool).await {
        Ok(payload) => HttpResponse::Ok().json(payload),
        Err(e) => {
            tracing::error!("An error occured when getting course by id:  {}", e);
            HttpResponse::BadGateway().finish()
        }
    }
}

#[tracing::instrument(
    name = "getting_course_by_id from database",
    skip(pool),
)]
pub async fn get_course_by_id(id: Uuid, pool: &PgPool) -> Result<Course, sqlx::Error>{
    let payload = sqlx::query_as!(
        Course,
        r#"SELECT * FROM courses WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await;
        payload
}
