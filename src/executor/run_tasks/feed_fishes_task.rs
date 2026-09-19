use reqwest::StatusCode;

use crate::models::Task;

pub async fn feed_fishes_v1(
    task: &str
) -> Result<String, StatusCode> 
{
    Ok(String::from("FEEDING FISHES FOR NOW"))
}