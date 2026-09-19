use std::println;

use reqwest::StatusCode;

use crate::{executor::run_tasks::feed_fishes_v1, models::Task};

pub async fn match_task(
    task: String
) -> Result<String, StatusCode>
{
    let res = match task.as_str() {
        "feed_fishes_v1" => {
            let resp = feed_fishes_v1(&task).await;
            let _ = match resp {
                Ok(val) => {
                    println!("FEED FISHES IS RETURNING THIS {}", val);
                }
                Err(err) => println!("{}", err)
            };
            Ok(String::from("found tasks"))
        }
        "test_email_v1" => {
            Ok(String::from("TESTING EMAIL"))
        }
        _ => Err(StatusCode::NOT_FOUND)
    };

    res
}