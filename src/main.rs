use std::{io, net::TcpListener};

use sqlx::postgres::PgPoolOptions;
use zero2prod::{configuration, startup, telemetry};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    let connection_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());
    startup::run(listener, connection_pool)?.await
}
