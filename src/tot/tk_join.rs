use std::{future::Future, pin::{pin, Pin}};

use tokio::{time:: {sleep, Duration}, sync::mpsc};


pub async fn tk_join_run() {

    channel().await;
    channel_mpsc().await;
    mpsc_join_macro().await;
    mpsc_join_vec().await;
}


async fn channel() {

    let (tx, mut rx) =  mpsc::channel(8);

    tokio::spawn(async move {
        for i in 0..10 {
            if let Err(_) = tx.send(i).await {
                println!("recever dropped");
                return;
            }
        }
    });

    while let Some(v) = rx.recv().await {
        if v == 5 {
            rx.close();
            // 这里如果不主动 return, channel 还会收到 6，7，8 不等，
            // 说明发送端不会立即收到接收端关闭的信号
            return;
        }
        println!("main::receiver: {v}");
    }

}


async fn channel_mpsc() {

    let (tx, mut rx) = mpsc::channel(8);

    // 此 handle1 代表的 Future 将会在 tx 发送完成之后完成
    // 如果仅仅时发送消息，无需获取所有权这里需要将 tx move 到异步代码块，
    // 使得异步代码结束的时候 tx 被丢弃,从而使得 接收端可以顺利结束
    //
    // let handle1 = async {
    //
    let handle1 =  async move{
        for i in 0..10 {
            if let Err(_) = tx.send(i).await {
                println!("mpsc::recever dropped");
                return;
            }
        }
    };

    // 接收端的 Future 在 tx 结束之后收到的都是 None
    // rx 则一直阻塞在 while let 循环中不会完成 (rx.close 或者 tx 被丢弃时才会结束)
    let handle2 = async  {
        while let Some(val) = rx.recv().await {
            println!("channel::receiver: {val}");
        }
        rx.close();
    };

    // join!() 结束的条件时内部两个 Future 都完成了。
    // 因为 rx 不会结束，因此 tokio::join!() 不会结束, 程序会一直阻塞, 不会结束
    tokio::join!(handle1, handle2);

}

//  join!()
//  > 如果需要处理 Future 返回不同 Output 情况 (即 Future 类型不同)
//  > 使用 join!() 处理固定数量 Future, 产生一个这些类型的元组.

async fn mpsc_join_macro() {
    let (tx, mut rx) = mpsc::channel(8);

    let tx1 = tx.clone();
    let tx1_fut = async move {
        for i in 0..10 {
            if let Err(_) = tx1.send(i).await {
                println!("recever dropped");
                return;
            }
            sleep(Duration::from_millis(500)).await;
        }
    };

    // Future 的创建顺序并不会对执行的顺序产生影响，真正决定代码执行的顺序的
    // 是执行的时候 await 的 顺序决定的.
    // 接收端放在前面并不会影响程序执行
    let rx1_fut = async {
        while let Some(v) = rx.recv().await {
            println!("mpsc::receiver: {v}");
        }
    };

    let tx_fut = async move {
        for i in 0..10 {
            if let Err(_) = tx.send(i).await {
                println!("recever dropped");
                return;
            }
            sleep(Duration::from_millis(1000)).await;
        }
    };

    tokio::join!(tx_fut, rx1_fut, tx1_fut);
}


// 
//  处理 Future 集合 join_all()
//
//  > 明确如果需要处理可变的集合，则需要明确 Future 都返回相同的 Output 类型, 使用 join_all()
//
async fn mpsc_join_vec() {

    let (tx, mut rx) = mpsc::channel(8);

    // 
    let tx_fut = pin!(async move {
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
        ];

        for val in vals {
            if let Err(_) = tx.send(val).await {
                println!("mpsc::sender dropped()");
                return;
            }
            sleep(Duration::from_millis(500)).await;
        }
    });

    // pin 住 Future, 得到的就是一个封装类型，因此可以放入 vec 中
    //
    let rx_fut = pin!(async {
        while let Some(val) = rx.recv().await {
            println!("mpsc::receiver: {val}");
        }
    });

    // 
    // vec  需要准确的知道元素的类型, 这是放入 vec 的基础
    // 因此我们需要确实的表示出 Pin 的引用类型
    //
    // > 持有一个 Pin 住的 动态 Future trait 对象可变引用
    //
    let list: Vec<Pin<&mut dyn Future<Output=()>>> = vec![tx_fut, rx_fut];

    trpl::join_all(list).await;
}
