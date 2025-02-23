use actix_web::{web, HttpResponse};
use sqlx::{types::chrono::Utc, PgPool};
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // Generate a random unique identifier for this request
    let request_id = Uuid::new_v4();
    // Using info_span! macro to create a new span and attach some values to its context
    // This macro creates a span at the info-level
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        // % symbol means use their `Display` implementation for logging purposes
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );
    // Use .enter() to activate the span, which returns a guard with type `Entered`
    // As long as the guard is not dropped, all downstream spans and log events will be
    // registed as children of the entered span
    let _request_span_guard = request_span.enter();

    // We do not call `.enter` on query_span!
    // `.instrument` takes care of it at the right moments
    // in the query future lifetime
    let query_span = tracing::info_span!("Saving new subscriber details in the database");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(pool.get_ref())
    // Attach the instrumentation then await it
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
