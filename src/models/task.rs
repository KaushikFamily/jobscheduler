use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskNextHourResponse {
    pub task_response: Option<Vec<TaskResponse>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskResponse {
    pub job_id: String,
    pub task_name: String,
    pub task_id: String,
    pub user_id: String,
    pub day: String,
    pub execute_time: String 
}

pub struct Task {
    pub job_id: String,
    pub task_name: String,
    pub task_id: String,
    pub user_id: String,
    pub day: String,
    pub execute_time: String
}