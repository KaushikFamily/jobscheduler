use std::println;

use reqwest::StatusCode;

use crate::{executor::run_tasks::{feed_fishes_v1, test_email_v1}, models::Task};

pub async fn match_task(
    task_name: String
) -> Result<String, StatusCode>
{
    let res = match task_name.as_str() {
        "feed_fishes_v1" => {
            let resp = feed_fishes_v1(&task_name).await;
            let _ = match resp {
                Ok(val) => {
                    println!("FEED FISHES IS RETURNING THIS {}", val);
                }
                Err(err) => println!("{}", err)
            };
            Ok(String::from("found tasks"))
        }
        "test_email_v1" => {
            let task_resp = test_email_v1().await;

            let _ = match task_resp {
                Ok(val) => {
                    println!("SUCCESSFULLY SENT test_email");
                }
                Err(err) => println!("Error sending email: {}", err)
            };
            
            Ok(String::from("TESTING EMAIL"))
        }
        _ => Err(StatusCode::NOT_FOUND)
    };

    res
}