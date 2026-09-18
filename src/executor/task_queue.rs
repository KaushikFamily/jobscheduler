use std::{collections::BinaryHeap, sync::{Arc, Mutex}};

use crate::models::TaskHeapNode;

pub type SharedTaskHeap = 
    Arc<Mutex<BinaryHeap<TaskHeapNode>>>;

pub fn new_queue() -> SharedTaskHeap {
    Arc::new(
        Mutex::new(BinaryHeap::new())
    )
}
    