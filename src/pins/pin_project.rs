//
// pin-project  / pin-project-lite 的辅助pin 对象的应用
// 

use std::{future::Future, task::ready};
use pin_project_lite::pin_project;


// 想要实现的结果
pub async fn use_pin_raw() {

    // 获取一个 Future 的引用
    // 自己手写实现
    let fut1 = foo();
    // 获取到的 Future 之后进行其他的操作
    let ret = Map::new(fut1, |i: u32| i * 2 ).await;
    println!("{}", ret);

    // -----------------------------------------------------------
    // pin-project-lite 库实现
    // let fut2 = foo();
    // let ret = PMap::new(fut2, |i: u32| i * 2 ).await;
    // println!("{}", ret);

    // 使用添加的扩展函数来完成调用
    let ret = foo().map(|i| i * 2).await;
    println!("{}", ret);
}

async fn foo() -> u32 {
    42
}


// 完成对应的 Map 的 定义
struct Map<Fut, F> {
    fut: Fut,
    f: Option<F>,
}

impl<Fut, F> Map<Fut, F> {
    fn new(fut: Fut, f: F) -> Self {
        Self { fut, f: Some(f) }
    }
}

impl <Fut, F, T> Future for Map<Fut, F> 
    where Fut: Future,
    F: FnOnce(Fut::Output) -> T
{
    type Output = T;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        
        // 因为无法知道 Fut 是否为 Unpin 类型，因此无法获取到可变引用
        // 使用 unsafe 获取 fut, 如下的写法 fut 还是无法调用 poll
        //
        // let fut  = unsafe{ &mut self.as_mut().get_unchecked_mut().fut };
        let fut = unsafe { self.as_mut().map_unchecked_mut(|map| &mut map.fut) };
        // ready!() 如果这个 fut 已经 ready 开始获取到值处理, 如果是 pending 则直接返回 pending
        let output = ready!(fut.poll(cx));

        // 获取 f 函数, 由于我们最终调用 f 函数需要获取 ownership, 因此这里需要 Option<F>
        let f = unsafe { &mut self.as_mut().get_unchecked_mut().f.take() };
        match f.take() {
            Some(fun) => std::task::Poll::Ready(fun(output)),
            None => panic!("call after completion"),
        }
    }
}


// -------------------------------------------------------
// pin-project   应用
//
// 1. 使用 pin_project!{ } 包裹对应的数据结构，#[pin] 标注 field 
// 2. 使用时，let this = self.project();
// 3. 获取对用 Pin 的字段 let fut = this.fut


// 优化PMap 的用法，使得更加符合 rust 风格

trait FutureExt: Future {
    fn map<F, T>(self, f: F) -> PMap<Self, F>
    where 
        F: FnOnce(Self::Output) -> T,
        Self: Sized
    {
        PMap::new(self, f)
    }
}

// 为所有实现了 Future 的类型实现 FutureExt 扩展
impl <T: Future> FutureExt for T {}


pin_project! {
    struct PMap<Fut, F> {
        #[pin]
        fut: Fut,
        f: Option<F>,
    }
}

impl<Fut, F> PMap<Fut, F> {
    fn new(fut: Fut, f: F) -> Self {
        Self { fut, f: Some(f) }
    }
}

impl <Fut, F, T> Future for PMap<Fut, F> 
    where Fut: Future,
    F: FnOnce(Fut::Output) -> T
{
    type Output = T;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {

        let this = self.project();

        // 因为无法知道 Fut 是否为 Unpin 类型，因此无法获取到可变引用
        // 使用 unsafe 获取 fut, 如下的写法 fut 还是无法调用 poll
        // let fut  = unsafe{ &mut self.as_mut().get_unchecked_mut().fut };
        //
        let fut = this.fut;
        // ready!() 如果这个 fut 已经 ready 开始获取到值处理, 如果是 pending 则直接返回 pending
        let output = ready!(fut.poll(cx));

        // 获取 f 函数, 由于我们最终调用 f 函数需要获取 ownership, 因此这里需要 Option<F>
        let f = this.f;
        match f.take() {
            Some(fun) => std::task::Poll::Ready(fun(output)),
            None => panic!("call after completion"),
        }
    }
}
