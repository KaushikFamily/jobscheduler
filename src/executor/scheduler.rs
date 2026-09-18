use std::collections::BinaryHeap;
use std::println;

use crate::{executor::{calc_seconds, task_queue::SharedTaskHeap}, models::{Task, TaskHeapNode}};

// let mut min_heap = BinaryHeap::new();

pub fn run_scheduler(
    queue: &SharedTaskHeap,
    tasks: Vec<Task>) -> Option<String> {
    
    let heap_nodes = tasks
        .iter()
        .map(|task| {
            TaskHeapNode {
                execute: calc_seconds(&task.execute_time).unwrap(), 
                task_name: task.task_name.clone()
            }
        })
        .collect::<Vec<TaskHeapNode>>();

    {
        let mut heap = queue.lock().unwrap();

        for node in heap_nodes {
            heap.push(node);
        }

        println!("PRODUCER ADDED ALL HEAP NODES");
    }
    
    Option::Some(String::from("PROCESS COMPLETED"))
}
