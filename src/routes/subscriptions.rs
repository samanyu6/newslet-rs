use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};
use chrono;
use serde;
use sqlx::PgPool;
use tracing;
use uuid::Uuid;

use crate::domain::{subscriber_name::NewSubscriber, SubscriberEmail, SubscriberName};

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;

        Ok(Self { email, name })
    }
}

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber"
    skip(_form, _connection)
    fields (
        subscriber_email = %_form.email,
        subscriber_name= %_form.name
    )
)]
pub async fn subscribe(_form: web::Form<FormData>, _connection: Data<PgPool>) -> impl Responder {
    let new_subscriber = match _form.0.try_into() {
        Ok(form) => form,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match insert_subscriber(&_connection, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subs, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, new_subs: &NewSubscriber) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subs.email.as_ref(),
        new_subs.name.as_ref(),
        chrono::Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
