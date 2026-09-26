use std::time::Duration;
use std::println;

use crate::{manager::{calc_seconds, get_next_hour_tasks, task_queue::SharedTaskHeap}, models::{Task, TaskHeapNode}};

// let mut min_heap = BinaryHeap::new();

pub async fn run_scheduler(
    queue: &SharedTaskHeap,
    sleep_time: u64
) 
{
    loop {

        let task_response = get_next_hour_tasks().await;
    
        let tasks_list  = 
            match task_response {
                Ok(tasks) => {
                    Some(tasks)
                }
                Err(status) => {
                    println!("Error status received: {}", status);
                    None
                }
            };
        
        match tasks_list {
            Some(tasks) => {
                let no_of_tasks = tasks.len();
                let _ = insert_into_queue(queue, tasks).await;
                println!("SCHEDULING {} TASKS", no_of_tasks)
            }
            None => println!("NO TASKS SCHEDULED FOR NEXT HOUR")
        }

        tokio::time::sleep(
            Duration::from_secs(sleep_time)
        ).await;
        
    }
}

async fn insert_into_queue(
    queue: &SharedTaskHeap, 
    tasks: Vec<Task>
) 
{
    let heap_nodes = tasks
        .iter()
        .map(|task| {
            TaskHeapNode {
                execute: calc_seconds(&task.execute_time).unwrap(), 
                job_id: task.job_id.clone(),
                task_name: task.task_name.clone()
            }
        })
        .collect::<Vec<TaskHeapNode>>();
    
    // Mutex locking for heap insertion
    {
        let mut heap = queue.lock().await;

        for node in heap_nodes {
            heap.push(node);
        }

        println!("SCHEDULER ADDED ALL HEAP NODES");
    }
}