use tokio::time::{ sleep, Duration };


pub async fn tk_race_run() {

    race_compare().await;
}


async fn race_compare() {

    let slow = async {
        println!("'slow' started.");
        sleep(Duration::from_millis(200)).await;
        println!("'slow' completed.");
    };

    let fast = async {
        println!("'fast' started.");
        sleep(Duration::from_millis(100)).await;
        println!("'fast' completed.");

    };
    trpl::race(slow, fast).await;

}
