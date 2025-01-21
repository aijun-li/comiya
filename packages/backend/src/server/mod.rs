use anyhow::Result;
use migration::{
    sea_orm::{Database, DatabaseConnection},
    Migrator, MigratorTrait,
};
use router::get_router;

mod middleware;
mod router;
mod types;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub async fn run() -> Result<()> {
    let db = Database::connect(env!("DATABASE_URL")).await?;

    Migrator::up(&db, None).await?;

    let state = AppState { db };

    let app = get_router().with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
