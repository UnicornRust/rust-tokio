use super::sleep_future::SleepFuture;
use crate::future::sleep_future::sleep_time;
use std::{future::Future, pin::Pin, task::Poll, time::Duration};

pub async fn async_run() {
    let v = vec![1, 2, 3];
    let s = String::from("hello");

    println!("----------------async--------------------");
    // foo(v, s).await;

    //
    // 使用自己写的 FooFut 来替代语法糖 async foo 的调用
    //
    let fut = FooFut::new(v, s);
    let res = fut.await;
    println!("{:?}", res);
}

async fn foo(v: Vec<u32>, s: String) -> u32 {
    println!("{:?}", v);
    sleep_time(Duration::from_secs(1)).await;
    println!("{s}");
    sleep_time(Duration::from_secs(1)).await;
    42
}

//
// 演示 async / await 底层在做什么
// 针对上述一段代码，底层生成执行了哪些代码
//
struct FooFut {
    state: FooState,
    v: Vec<u32>,
    s: String,
}

enum FooState {
    Init,
    Sleep1(SleepFuture),
    Sleep2(SleepFuture),
    Done,
}

impl FooFut {
    pub fn new(v: Vec<u32>, s: String) -> Self {
        Self {
            state: FooState::Init,
            v,
            s,
        }
    }
}

impl Future for FooFut {
    type Output = u32;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        loop {
            match self.as_mut().get_mut().state {
                FooState::Init => {
                    println!("{:?}", self.v);
                    let fut1 = SleepFuture::new(Duration::from_secs(1));
                    self.as_mut().get_mut().state = FooState::Sleep1(fut1);
                }
                FooState::Sleep1(ref mut fut1) => match Pin::new(fut1).poll(cx) {
                    Poll::Ready(_) => {
                        println!("{}", self.s);
                        let fut2 = SleepFuture::new(Duration::from_secs(1));
                        self.as_mut().get_mut().state = FooState::Sleep2(fut2);
                    }
                    Poll::Pending => {
                        return Poll::Pending;
                    }
                },
                FooState::Sleep2(ref mut fut2) => match Pin::new(fut2).poll(cx) {
                    Poll::Ready(_) => {
                        self.as_mut().get_mut().state = FooState::Done;
                    }
                    Poll::Pending => {
                        return Poll::Pending;
                    }
                },
                FooState::Done => {
                    return Poll::Ready(42);
                }
            }
        }
    }
}
