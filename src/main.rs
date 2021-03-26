// mod store;
mod challenge;

use warp::{http, Filter};
use std::thread;
use challenge::{Item, ItemResult, MAX_MSGS, N_WORKERS};
use crossbeam_channel::{Sender, Receiver, unbounded, TryRecvError};

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {
    let (send_item, recv_item) = unbounded();
    let (send_result, recv_result) = unbounded();

    thread::spawn(move || run_workers(recv_item, send_result));

    let add_tasks 
    = warp::post()
        .and(warp::path("add"))
        .and(warp::path::end())
        .and(warp::body::json())
        .map(move |item| (item, send_item.clone()))
        .and_then(add_item);

    let get_results 
    = warp::get()
        .and(warp::path!("results"))
        .map(move || recv_result.clone())
        .and_then(get_results);

    let routes = add_tasks.or(get_results);

    println!("Starting quick_compute");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn add_item((item, channel): (Item, Sender<Item>)) -> Result<impl warp::Reply, warp::Rejection> {
    // task_store.add(item);
    match channel.send(item) {
        Ok(_) => Ok(warp::reply::with_status(
            "Added task",
            http::StatusCode::CREATED,
        )),
        Err(_) => Err(warp::reject::reject())
    }
}

async fn get_results(channel: Receiver<ItemResult>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut results = Vec::new();
    for _ in 0..MAX_MSGS {
        match channel.try_recv() {
            Ok(res) => results.push(res),
            Err(TryRecvError::Empty) => break,
            _ => return Err(warp::reject::reject())
        }
    } 

    Ok(warp::reply::json(&results))
}

fn run_workers(recv_item: Receiver<Item>, send_result: Sender<ItemResult>) {
    for _ in 0..N_WORKERS {
        let (s, r) = (send_result.clone(), recv_item.clone());
        thread::spawn(move || {
            for item in r.iter() {
                if let Err(e) = s.send(challenge::process(item)) {
                    eprintln!("Error on sending ItemResult {:?} | Killing thread", e);
                    break
                }
            }
        });
    }
}