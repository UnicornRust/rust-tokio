use tokio::sync::mpsc;


pub async fn channel_mpsc() {

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
        println!("receiver: {v}");
    }

}

