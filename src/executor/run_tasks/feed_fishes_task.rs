use reqwest::StatusCode;

use crate::models::Task;

pub async fn feed_fishes_v1(
    task: Task
) -> Result<String, StatusCode> 
{
    Ok(String::from("returning this stub for now"))
}