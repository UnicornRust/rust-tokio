use std::time::Duration;

use async_await::async_run;
use sleep_future::sleep_time;

pub mod sleep_future;
pub mod async_await;

pub async fn run() {
    sleep_time(Duration::from_secs(1)).await;
    async_run().await;
}
