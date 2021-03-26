use serde::{Deserialize, Serialize};
use std::thread;

pub const N_WORKERS: u16 = 4;
pub const MAX_MSGS: u16 = 100;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {

}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ItemResult {

}

pub fn process(item: Item) -> Option<ItemResult> {
    println!("{:?} || Processing {:?}", thread::current().id(), item);
    None
}