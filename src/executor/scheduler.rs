use std::collections::BinaryHeap;
use std::println;

use crate::models::{Task, TaskHeapNode};

// let mut min_heap = BinaryHeap::new();

pub async fn run_scheduler(tasks: Vec<Task>) -> Option<String> {
    
    let heap_nodes = tasks
        .iter()
        .map(|task| {
            TaskHeapNode {
                execute: calc_seconds(&task.execute_time).unwrap(), 
                task_name: task.task_name.clone()
            }
        })
        .collect::<Vec<TaskHeapNode>>();

    let mut heap: BinaryHeap<TaskHeapNode> = BinaryHeap::new();
    
    for node in heap_nodes {
        heap.push(node);
    }

    while let Some(top) = heap.pop() {
        println!("{}", top.execute)
    }

    Option::Some(String::from("PROCESS COMPLETED"))
}

fn calc_seconds(time: &str) -> Result<u32, std::num::ParseIntError> {
    let parts: Vec<u32> = time
        .split(':')
        .map(|x| x.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()?;

    Ok(parts[0] * 3600 + parts[1] * 60 + parts[2])
}