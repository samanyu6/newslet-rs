use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};
use chrono::Utc;
use sqlx::PgPool;
use tracing::{info_span, Instrument};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(_form: web::Form<FormData>, _connection: Data<PgPool>) -> impl Responder {
    let request_id = Uuid::new_v4();

    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %_form.email,
        subscriber_name= %_form.name
    );

    let _req_span_guard = request_span.enter();

    let query_poll = tracing::info_span!("saving new subscriber details");

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        _form.email,
        _form.name,
        Utc::now()
    )
    .execute(_connection.get_ref())
    .instrument(query_poll)
    .await
    {
        Ok(_) => {
            tracing::info!("request id {}: New subscriber details saved", request_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "request id {}: Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
