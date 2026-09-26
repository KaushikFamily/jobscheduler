use std::{println, ptr::null};

use crate::models::{Job, JobRequest, JobResponse};

use axum::{
    Json, extract::State, http::StatusCode, response::IntoResponse,
};

// 1. Return a Result so the `?` operator works. 
// 2. Use `impl IntoResponse` so you can return either Success (Json) or Failure (StatusCode).
pub async fn create_job(
    Json(payload): Json<JobRequest>
) -> Result<(StatusCode, Json<Option<JobResponse>>), StatusCode> {
    
    let client = reqwest::Client::new();

    let response = client
        .post("http://localhost:8080/jobs/schedule")
        .json(&payload)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; 

    if response.status().is_success() {
        // let response_text = &response.text().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        // println!("RAW JSON RECEIVED: {}", response_text);

        let job_response: JobResponse = response.json()
            .await
            .map_err(|e| {
                println!("Deserialization error: {:?}", e); 
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            
        println!("successfully parsed job. ID: {}", job_response.job.job_id);
        
        Ok((StatusCode::CREATED, Json(Some(job_response))))
    } else {
        println!("API err: {}", response.status());
        // Return whatever failure code the backend server gave us
        Err(StatusCode::from_u16(response.status().as_u16())
            .unwrap_or(StatusCode::BAD_REQUEST))
    }
}

