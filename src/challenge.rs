use serde::{Deserialize, Serialize};
use std::thread;

pub const N_WORKERS: u16 = 8;
pub const MAX_MSGS: u16 = 100;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {
    state: usize,
    target_hash: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ItemResult {
    item: Item,
    success: bool,
}

pub fn process(item: Item) -> ItemResult {
    println!("{:?} || Processing {:?}", thread::current().id(), item);
    ItemResult {
        item,
        success: false
    }
}