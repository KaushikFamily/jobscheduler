use reqwest::StatusCode;

use crate::models::Task;


pub async fn match_task(
    task: Task
) -> Result<String, StatusCode>
{
    let res = match task.task_name.as_str() {
        "feed_fishes_v1" => {
            // TODO
            Ok(String::from("found tasks"))
        }
        _ => Err(StatusCode::NOT_FOUND)
    };

    res
}