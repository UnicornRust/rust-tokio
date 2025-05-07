use base::tokio_fn;
use channel::channel_mpsc;

// tokia 的 编排
pub mod base;
pub mod channel;


pub async fn run() {
    // tokio_fn().await;
    channel_mpsc().await;
}
