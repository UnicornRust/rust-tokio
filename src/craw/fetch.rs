use std::{future::Future, process::Output};

use trpl::{Either, Html};


pub async fn run() {

    let t2 = page_title("https://www.baidu.com");
    let t1 = page_sync("https://apple.com");

    let title = match trpl::race(t1, t2).await {
        Either::Left(left) => left,
        Either::Right(right) => right,
    };

    // println!("{url} return first");

    match title {
        Some(title) => println!("It's page title is {title}"),
        None => println!("It's title could not be parsed"),
    }

}

async fn page_title(url: &str) -> Option<String> {
    // let response_value = trpl::get(url).await;
    // let response = response.text().await;
    
    // rust 的 await 属于后缀关键字，这样使得链式操作更加便捷
    // 如上语句可以使用链式调用
    let response = trpl::get(url).await.text().await;
    Html::parse(&response)
        .select_first("title")
        .map(|title| title.inner_html())
}

// 上述异步代码在编译之后被翻译为如下的代码
fn page_sync(url: &str) -> impl Future<Output = Option<String>> + '_ {
    async move {
        let response = trpl::get(url).await.text().await;
        Html::parse(&response)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
