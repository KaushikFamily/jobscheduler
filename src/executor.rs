use std::println;

use axum::Json;
use reqwest::StatusCode;

use crate::models::{TaskNextHourResponse, TaskResponse};


pub async fn get_next_hour_tasks() -> 
Result<(StatusCode, Json<Option<TaskNextHourResponse>>), StatusCode> {

    let client = reqwest::Client::new();
    
    let response = client
        .get("http://localhost:8080/tasks/getNextHour")
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if response.status().is_success() {

        let tasks_response: TaskNextHourResponse = response.json()
            .await
            .map_err(|e| {
                println!("Deserialization error: {:?}", e); 
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        match &tasks_response.task_response {
            Some(tasks) => {
                for task in tasks {
                    println!("successfully parsed job. TASK_NAME: {}", task.task_name);
                }
        }
            None => println!("NO TASKS SCHEDULED IN NEXT HOUR")
        }
        
        Ok((StatusCode::CREATED, Json(Some(tasks_response))))
    } else {
        println!("API err: {}", response.status());
        // Return whatever failure code the backend server gave us
        Err(StatusCode::from_u16(response.status().as_u16())
            .unwrap_or(StatusCode::BAD_REQUEST))
    }


}