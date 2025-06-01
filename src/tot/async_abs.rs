use std::{future::Future, time::Duration};
use tokio::time::sleep;
use trpl::Either;


pub async fn timeout_run() {

    let slow = async {
        sleep(Duration::from_millis(100)).await;
        "I finished!"
    };

    match timeout(slow, Duration::from_millis(10)).await {
        Ok(message) => println!("Successed with {}", message),
        Err(duration) => {
            println!("timeout after {}", duration.as_millis());
        }
    }
}

async fn timeout<F: Future>(
    future_to_try: F, 
    max_time: Duration
) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time)
    }
}
