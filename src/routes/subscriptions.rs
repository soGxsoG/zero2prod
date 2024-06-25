use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct FormData{
    email: String,
    name: String
}


pub async fn subscribe(
    _form: web::Form<FormData>,
    _pool : web::Data<PgPool>
) -> HttpResponse {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        _form.email,
        _form.name,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.  
    .execute(_pool.get_ref())
    .await;
    HttpResponse::Ok().finish()
}