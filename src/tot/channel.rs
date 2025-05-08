use tokio::{time:: {sleep, Duration}, sync::mpsc};


pub async fn channel() {

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

pub async fn mpsc_join() {

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


pub async fn mpsc_join_order() {
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

