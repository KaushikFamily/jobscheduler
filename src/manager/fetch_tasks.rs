use std::println;

use reqwest::StatusCode;

use crate::models::{Task, TaskNextHourResponse};

pub async fn get_next_hour_tasks() -> Result<Vec<Task>, StatusCode> {

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

        let tasks = match &tasks_response.task_response {
            Some(tasks) => {
                tasks
                    .iter()
                    .map(|task| {
                        println!(
                            "successfully parsed job. TASK_NAME: {}",
                            task.task_name
                        );

                        Task {
                            job_id: task.job_id.clone(),
                            task_name: task.task_name.clone(),
                            task_id: task.task_id.clone(),
                            user_id: task.user_id.clone(),
                            day: task.day.clone(),
                            execute_time: task.execute_time.clone(),
                        }
                    })
                    .collect::<Vec<Task>>()
            }
            None => {
                println!("NO TASKS SCHEDULED IN NEXT HOUR");
                Vec::new()
            }
        };
        Ok(tasks)
    } else {
        println!("API err: {}", response.status());
        // Return whatever failure code the backend server gave us
        Err(StatusCode::from_u16(response.status().as_u16())
            .unwrap_or(StatusCode::BAD_REQUEST))
    }
}