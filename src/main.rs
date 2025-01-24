
use tokio::time::{ sleep, timeout, Duration };


#[tokio::main]
async fn main() {

    println!("Hello tokio");
    base().await;
    multi_task().await;
    timeout_task().await;
}

async fn base() {
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

    let _ = tokio::join!(task_one, task_two);

}

async fn process_task(id: u32) {
    println!("Task { } is stated", id);
    sleep(Duration::from_secs(2)).await;
    println!("Task {} has completed", id);
}

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
    let result = timeout(Duration::from_secs(1), process_task(2)).await;
    match result {
        Ok(_) => println!("Task completed within time"),
        Err(_) => println!("Task timeout"),
    }
}
