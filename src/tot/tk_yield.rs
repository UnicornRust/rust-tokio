use std::{thread::sleep, time::Duration};

use tokio::time::Instant;
use trpl::yield_now;


// 异步代码在只会在一个 await point 暂停异步代码块，并将控制权交还给运行时
// 在 await point 之间的一切都是同步运行的
//
// 1. 这就意味着，当一个异步代码块中存在一个运行很长时间(或者说阻塞)
//    则这个future 会阻塞其他任何 future 继续执行 
// 2. 在一些需要高吞吐并发的环境下，一个长时间运行的任务或者无限持续运行的
//    任务在一个合适的时机交还控制权给运行时是非常重要的.
// 3. 如果存在长时间运行的阻塞操作，异步可能是一个提供了将程序的不同部分
//    相互关联起来的使用工具

fn slow(name: &str, duration: u64) {
    // thread::sleep 会阻塞整个代码运行
    // 模仿现实中的长时间阻塞操作, 
    sleep(Duration::from_millis(duration));
    println!("{name} run for {duration}ms")
}


pub async fn tk_yield_run() {
    long_pice_wait().await;
    small_pice_wait().await;
    yield_wait().await;
    yield_benchmark().await;
}

// 从这个程序中我们可以看出，两个程序并没有真正的并发运行,而是 
// a future 在运行到第一个 await point 的时候，让出了执行权限
// b future 才开始执行，这时候 a 并没有在时间到达的时候完成程序，
// 而是需要等到 b 执行到第一个 await point 交出控制权给运行时 
// 之后才会检查 a 的状态, 至此, a future ready 了，整个程序结束.

async fn long_pice_wait() {

    let a = async {
        let a = "a";
        println!("{a} started");
        slow(a, 30);
        slow(a, 10);
        slow(a, 20);
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        println!("{a} finished.");
    };

    let b = async {
        let b = "b";
        println!("{b} started");
        slow(b, 75);
        slow(b, 10);
        slow(b, 350);
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        println!("{b} finished.");
    };
    trpl::race(a, b).await;
}

// 手动制造 await point 

async fn small_pice_wait() {

    let one_ms = tokio::time::Duration::from_millis(1);

    let a = async {
        let a = "a";
        println!("{a} started");
        slow(a, 30);
        tokio::time::sleep(one_ms).await;
        slow(a, 10);
        tokio::time::sleep(one_ms).await;
        slow(a, 20);
        tokio::time::sleep(one_ms).await;
        println!("{a} finished.");
    };

    let b = async {
        let b = "b";
        println!("{b} started");
        slow(b, 75);
        tokio::time::sleep(one_ms).await;
        slow(b, 10);
        tokio::time::sleep(one_ms).await;
        slow(b, 350);
        tokio::time::sleep(one_ms).await;
        println!("{b} finished.");
    };
    trpl::race(a, b).await;
}

// 使用 yield 来主动交还控制权
async fn yield_wait() {
    let a = async {
        let a = "a";
        println!("{a} started");
        slow(a, 30);
        trpl::yield_now().await;
        slow(a, 10);
        trpl::yield_now().await;
        slow(a, 20);
        trpl::yield_now().await;
        println!("{a} finished.");
    };

    let b = async {
        let b = "b";
        println!("{b} started");
        slow(b, 75);
        trpl::yield_now().await;
        slow(b, 10);
        trpl::yield_now().await;
        slow(b, 350);
        trpl::yield_now().await;
        println!("{b} finished.");
    };
    trpl::race(a, b).await;

}


// yield 与 sleep 之间的性能对比
async fn yield_benchmark() {

    let one_ns = tokio::time::Duration::from_nanos(1);
    let start = Instant::now();
    async {
        for _ in 1..1000 {
            tokio::time::sleep(one_ns).await;
        }
    }.await;

    println!("sleep benchmark took {}ns", start.elapsed().as_nanos());

    let start = Instant::now();
    async {
        for _ in 1 .. 1000 {
            trpl::yield_now().await
        }
    }.await;
    println!("yield benchmark took {}ns", start.elapsed().as_nanos());
}


