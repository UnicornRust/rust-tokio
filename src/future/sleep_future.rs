
use std::{future::Future, sync::{Arc, Mutex}, task::{Poll, Waker}, thread, time::Duration};

// 使用 sleep_future 来说明 Future 中 poll 的调用过程
// Poll::Pending / Poll::Ready 之间状态的转换
// wake 的使用可以使得 Future 主动通知 AsyncRuntime 继续执行

pub async fn sleep_time(duration: Duration) {
    SleepFuture::new(duration).await
}


pub struct SleepFuture {
    duration: Duration,
    state: Arc<Mutex<State>>,
}

struct State  {
    waker: Option<Waker>,
    inner_state: InnerState,
}

#[derive(PartialEq)]
enum InnerState {
    Init,
    Sleeping,
    Done,
}

impl SleepFuture {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            state: Arc::new(Mutex::new(State{
                waker: None,
                inner_state: InnerState::Init
            }))
        }
    }
}

impl Future for SleepFuture {

    type Output = ();

    // poll 可能会进入多次
    //
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let mut guard = self.state.lock().unwrap();

        println!("poll......");

        if guard.inner_state == InnerState::Done {
            return Poll::Ready(());
        }

        if guard.inner_state == InnerState::Init {
            guard.waker = Some(cx.waker().clone());
            guard.inner_state  = InnerState::Sleeping;

            let duration = self.duration;
            let state_cloen = Arc::clone(&self.state);

            thread::spawn(move || {
                println!("start sleeping......");
                thread::sleep(duration);
                let mut guard = state_cloen.lock().unwrap();
                guard.inner_state = InnerState::Done;
                if let Some(waker) = guard.waker.take() {
                    waker.wake();
                }
                println!("end sleeping......");
            });
        }
        guard.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}


