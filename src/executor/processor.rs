use std::{println, thread, time::Duration};

use crate::executor::{current_time_seconds, run_tasks::match_task, task_queue::SharedTaskHeap};

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
                    let _ = match_task(node.task_name).await;
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
