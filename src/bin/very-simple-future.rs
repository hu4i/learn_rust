use learn_rust::tokio_tutorial::*;
use std::time::{Duration, Instant};



#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_millis(10);

    let future = Delay { when };

    let out = future.await;
    assert_eq!(out, "done");
}
