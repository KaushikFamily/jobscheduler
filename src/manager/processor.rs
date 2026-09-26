use std::{println, thread, time::Duration};

use crate::{manager::{current_time_seconds, drop_event, match_task, task_queue::SharedTaskHeap}, models::Event};

pub async fn process_tasks(
    queue: SharedTaskHeap,
    sleep_time: u64
) 
{
    loop {
        let current_time = current_time_seconds();

        loop {
            let task = {
                let mut heap = queue.lock().await;

                match heap.peek() {
                    Some(top) => {
                        if current_time >= top.execute {
                            heap.pop()
                        } else {
                            None
                        }
                    } 
                    _ => None,
                }
            };

            match task {
                Some(node) => {
                    println!("PROCESSSING NODE {} at time {}", node.task_name, node.execute);
                    
                    let event = Event {
                        job_id: node.job_id,
                        job_name: node.task_name,
                        event_type: String::from("CONSUME")
                    };

                    let _ = drop_event(event).await;
                },
                None => {
                    break;
                }
            }
        }

        println!("PROCESSOR SLEEPING FOR {} SECONDS", sleep_time);

        tokio::time::sleep(
            Duration::from_secs(sleep_time)
        ).await;
    }
}
