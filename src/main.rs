use std::println;

mod routes;
mod schedule_job;
mod models;

#[tokio::main]
async fn main() {
    let app = routes::create_routes();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("SERVER IS RUNNING ON HTTP://127.0.0.1:3000");

    
    axum::serve(listener, app).await.unwrap();
}
