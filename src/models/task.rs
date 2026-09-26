use std::cmp::Ordering;

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

#[derive(Debug, Eq, PartialEq)]
pub struct TaskHeapNode {
    pub execute: u32,
    pub job_id: String,
    pub task_name: String
}

// ********** Trait Implementation **********

impl Ord for TaskHeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.execute.cmp(&self.execute)
    }
}

impl PartialOrd for TaskHeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ********** Trait Implementation **********