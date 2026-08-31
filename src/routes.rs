use axum::{
    routing:: {get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct JobRequest {
    job_name: String,
}

#[derive(Serialize)]
pub struct JobResponse {
    pub message: String,
}

async fn hello_world() -> &'static str {
    "Hello, World"
}

async fn create_job(Json(payload): Json<JobRequest>) -> Json<JobResponse> {
    let response = JobResponse {
        message: format!("Job '{}' successfully created!", payload.job_name)
    };

    Json(response)
}

pub fn create_routes() -> Router {
    Router::new()
    .route("/", get(hello_world))
    .route("/jobs", post(create_job))
}
