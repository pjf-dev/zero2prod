use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;

use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to read configuration.");

    let pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let addr = format!("127.0.0.1:{}", config.application_port);

    let listener = TcpListener::bind(addr)?;
    run(listener, pool)?.await
}
