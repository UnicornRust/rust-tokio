use base::tokio_fn;
use channel::{channel, mpsc_join, mpsc_join_order};

// tokia 的 编排
pub mod base;
pub mod channel;


pub async fn run() {
    // tokio_fn().await;
    // mpsc_join().await;
    // channel_mpsc().await;
    mpsc_join_order().await;
}
