use rs_tokio::{craw, fsapi, future, pins, tot};


#[tokio::main]
async fn main() {

    println!("Hello tokio");
    // 
    // future 运行规则
    //
    // future::run().await;

    // 
    // pins 运行规则
    //
    // pins::run().await;

    // craw data
    // craw::fetch::run().await;

    // tokio api 
    tot::run().await;



    // tokio fs mod
    // fsapi::fileop::fs_sync().await;
}
