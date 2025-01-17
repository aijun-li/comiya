use router::get_router;

mod middleware;
mod router;
mod types;

pub async fn run() {
    let app = get_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
