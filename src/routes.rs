use crate::schedule_job::create_job;

use axum::{
    Json, Router,
    routing::{get, post},
};

use serde::{Deserialize, Serialize};

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
