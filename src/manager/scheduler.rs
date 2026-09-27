use std::{collections::HashSet, time::Duration};
use std::println;

use reqwest::StatusCode;

use crate::{manager::{calc_seconds, get_next_hour_tasks, task_queue::SharedTaskHeap}, models::{Task, TaskHeapNode}};

// let mut min_heap = BinaryHeap::new();

pub async fn run_scheduler(
    queue: &SharedTaskHeap,
    sleep_time: u64,
    completed_job_ids: &mut HashSet<String>
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
                let _ = insert_into_queue(queue, tasks, completed_job_ids).await;
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
    tasks: Vec<Task>,
    completed_job_ids: &mut HashSet<String>
)
{
    let heap_nodes = tasks
        .iter()
        .map(|task| {
            if (!completed_job_ids.contains(&task.job_id)) {
                Some(TaskHeapNode {
                    execute: calc_seconds(&task.execute_time).unwrap(), 
                    job_id: task.job_id.clone(),
                    task_name: task.task_name.clone()
                })
            } else {
                None
            }
        })
        .collect::<Vec<Option<TaskHeapNode>>>();

    let mut no_added = 0;
    
    // Mutex locking for heap insertion
    {
        let mut heap = queue.lock().await;

        let no_nodes_added = heap_nodes
            .into_iter()
            .map(|node| {
                match node {
                    Some(data) => {
                        completed_job_ids.insert(data.job_id.clone());
                        heap.push(data);
                        1
                    }
                    None => 0,
                } 
        })
        .collect::<Vec<i32>>();

        no_added = no_nodes_added.iter().sum();

        println!("SCHEDULER ADDED ALL HEAP NODES");
    }

    println!("SCHEDULING {} TASKS", no_added)
}