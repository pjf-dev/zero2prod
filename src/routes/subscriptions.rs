use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SubscribeForm {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<SubscribeForm>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let query = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        values ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await;
    match query {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprint!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
