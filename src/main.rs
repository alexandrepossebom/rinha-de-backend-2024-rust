use axum::{
    routing::{get, post},
    Router,
};

use handlers::{add_transaction, get_account};
use sqlx::postgres::PgPoolOptions;

pub mod apperror;
pub mod handlers;
pub mod models;

#[tokio::main]
async fn main() {
    let dsn = std::env::var("DATABASE_URL").unwrap();
    let port = std::env::var("PORT")
        .unwrap_or("9999".to_string())
        .parse::<u16>()
        .unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(10)
        .connect(dsn.as_str())
        .await
        .unwrap();

    let app = Router::new()
        .route("/clientes/:id/transacoes", post(add_transaction))
        .route("/clientes/:id/extrato", get(get_account))
        .with_state(pool);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
