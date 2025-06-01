use std::{thread::sleep, time::Duration};

use trpl::{ReceiverStream, Stream, StreamExt};


// stram api 
pub async fn stream_run() {
    iter_to_stream().await;
    compose_stream().await;
}

async fn iter_to_stream() {

    let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = values.iter().map(|n | n * 2);
    let mut stream = trpl::stream_from_iter(iter);

    while let Some(item) = stream.next().await {
        println!("The value was : {item}");
    }
}

async fn compose_stream() {
    let mut messages = get_message();
    while let Some(message) = messages.next().await {
        println!("${message}");
    }
}

fn get_message() -> impl Stream<Item = String>  {
    let ( tx, rx ) = trpl::channel(); 

    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    for message in messages {
        tx.send(format!("message:'{message}'")).unwrap();
    }
    ReceiverStream::new(rx)
}

// --------------------------------------------------------
// 为流增加延迟后出现超时的操作

fn get_messages_random_time() -> impl Stream<Item = String> {
    let ( tx, rx ) = trpl::channel(); 
    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    trpl::spawn_task(async move {
        for (index, message) in messages.into_iter().enumerate() {
            let sleep_time = if index % 2 == 0 { 100 } else { 200 };
            trpl::sleep(Duration::from_millis(sleep_time)).await;
            tx.send(format!("message:'{message}'")).unwrap();
        }
    });
    ReceiverStream::new(rx)
}
