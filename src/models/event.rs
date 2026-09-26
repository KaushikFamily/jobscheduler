use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub event_type: String,
    pub job_id: String,
    pub job_name: String
}