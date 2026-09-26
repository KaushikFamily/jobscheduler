use crate::schedule_job::create_job;

use axum::{
    Router,
    routing::{get, post},
};

async fn hello_world(
) -> &'static str 
{
    "Hello, World"
}

pub fn create_routes(
) -> Router 
{
    Router::new()
        .route("/", get(hello_world))
        .route("/jobs/schedule", post(create_job))
}
