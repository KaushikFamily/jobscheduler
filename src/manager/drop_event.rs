// Drops event for JobExecutor to handle

use axum::Json;
use reqwest::{Client, StatusCode};

use crate::models::Event;

pub async fn drop_event(
    event: Event
) -> Result<(StatusCode, String), StatusCode> 
{
    let client = Client::new();
    let url = "http://localhost:3001/consume";

    let response = client
        .post(url)
        .json(&event)
        .send()
        .await
        .map_err(|err| {
            println!("this is the err: {}", err.is_connect());
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    if response.status().is_success() {
        println!("EVENT SUCCESSFULLY DROPPED");
        Ok((StatusCode::OK, String::from("EVENT SUCCESSFULLY DROPPED")))
    } 
    else
    {
        println!("API err: {}", response.status());
        // Return whatever failure code the backend server gave us
        Err(StatusCode::from_u16(response.status().as_u16())
            .unwrap_or(StatusCode::BAD_REQUEST))
    }
}