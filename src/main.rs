use std::{collections::HashSet, println, sync::Arc};

use crate::{manager::{new_queue, process_tasks, run_scheduler, task_queue::SharedTaskHeap}};

mod routes;
mod schedule_job;
mod models;
mod manager;

#[tokio::main]
async fn main() {
    let app = routes::create_routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("SERVER IS RUNNING ON HTTP://0.0.0.0:3000");

    // Creating mutex lock on shared binary heap
    let queue : SharedTaskHeap = new_queue();

    // Copying mutex lock to share heap across threads
    let scheduler_heap: SharedTaskHeap = Arc::clone(&queue);
    let executor_heap: SharedTaskHeap = Arc::clone(&queue);

    let scheduler_thread = tokio::spawn(async move {
        let mut completed_job_ids: HashSet<String> = HashSet::new();
        run_scheduler(&scheduler_heap, 10, &mut completed_job_ids).await;
    });

    let executor_thread = tokio::spawn(async move {
        process_tasks(executor_heap, 5).await
    });

    let server_thread = tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .unwrap();
    });

    let _ = scheduler_thread.await;
    let _ = executor_thread.await;
    let _ = server_thread.await;

}
