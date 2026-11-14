
use axum::{
    routing::get,
    Router,
};
use sqlx::postgres::PgPoolOptions;

mod models {
    pub mod user;
}

mod repositories {
    pub mod user_repository;
}

mod services {
    pub mod user_service;
}

mod controllers {
    pub mod user_controller;
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    let app = Router::new()
        .route(
            "/api/v1/users",
            get(controllers::user_controller::get_users)
                .post(controllers::user_controller::create_user),
        )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(
        "127.0.0.1:8080"
    ).await?;

    println!("Rust backend running on http://127.0.0.1:8080");

    axum::serve(listener, app).await?;

    Ok(())
}
