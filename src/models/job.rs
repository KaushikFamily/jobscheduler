use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    #[serde(rename = "userId")]
    pub user_id: String,

    #[serde(rename = "jobId")]
    pub job_id: String,

    #[serde(rename = "taskName")]  
    pub task_name: String,

    #[serde(rename = "retryCount")]
    pub retry_count: i32,
    
    #[serde(rename = "createdAt")]
    pub created_at: String,
    
    #[serde(rename = "statusId")]
    pub status_id: String,
    
    #[serde(rename = "lastCompletedTime")]
    pub last_completed_time: Option<String>,
    
    #[serde(rename = "executedCount")]
    pub executed_count: i32,
    
    #[serde(rename = "failureCount")]
    pub failure_count: i32,
    
    pub repeating: bool,
    pub decommissioned: bool
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // <--- This handles all fields automatically!
pub struct JobRequest {
    pub user_id: String,     // Becomes "userId"
    pub task_name: String,   // Becomes "taskName"
    pub repeating: bool,     // Remains "repeating"
    pub retry_count: i32,    // Becomes "retryCount"
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobResponse{
    pub job: Job
}
