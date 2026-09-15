use std::println;

use crate::executor::processor;

mod routes;
mod schedule_job;
mod models;
mod executor;

#[tokio::main]
async fn main() {
    let app = routes::create_routes();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("SERVER IS RUNNING ON HTTP://127.0.0.1:3000");

    let task_response = executor::get_next_hour_tasks().await;
    
    match task_response {
        Ok(tasks) => {
            let _ = processor::process_tasks(tasks).await;
        }
        Err(status) => println!("Error status received: {}", status)
    }
 
    axum::serve(listener, app).await.unwrap();
}
