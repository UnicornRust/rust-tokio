use tokio::time::{ sleep,timeout, Duration };

pub async fn tokio_fn() {
    base().await;
    multi_task().await;
    timeout_task().await;
}

// 用法 
// --------------------
// tokio::spawn 
// tokio::join 
//
async fn base() {
    //
    // spawn 接收一个 Future (async{} 返回了一个 Future) 作为参数, 
    // tokio 开启一个新运行线程
    //
    let task_one = tokio::spawn(async {
        println!("Task one is started");
        sleep(Duration::from_secs(2)).await;
        println!("Task one has completed");
    });

    let task_two = tokio::spawn(async {
        println!("Task two is started");
        sleep(Duration::from_secs(1)).await;
        println!("Task two has completed");
    });

    // 让两个线程同时执行
    let _ = tokio::join!(task_one, task_two);
}

// 
//  tokio::spawn(async {})
// 
async fn multi_task() {

    let mut handles = vec![];

    for i in 0..5 {
        let handle = tokio::spawn(process_task(i));
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.await;
    }
}

async fn timeout_task()  {
    let result = timeout(
        Duration::from_secs(1), 
        process_task(10)
    ).await;
    match result {
        Ok(_) => println!("Task completed within time"),
        Err(_) => println!("Task timeout"),
    }
}

async fn process_task(id: u32) {
    println!("Task { } is stated", id);
    sleep(Duration::from_secs(2)).await;
    println!("Task {} has completed", id);
}
