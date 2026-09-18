use std::{collections::BinaryHeap, println, sync::Arc};

use tokio::sync::Mutex;

use crate::{executor::scheduler, models::TaskHeapNode};

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

    let mut min_heap: BinaryHeap<TaskHeapNode> = BinaryHeap::new();

    let heap = Arc::new(Mutex::new(min_heap));

    let task_response = executor::get_next_hour_tasks().await;
    
    match task_response {
        Ok(tasks) => {
            if tasks.len() != 0 {
                let _ = scheduler::run_scheduler(tasks);
            } else {
                println!("EMPTY LIST: PROCESSING_TASKS SKIPPED")
            }
        }
        Err(status) => println!("Error status received: {}", status)
    }
 
    axum::serve(listener, app).await.unwrap();
}
