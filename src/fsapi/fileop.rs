use std::fs;

pub async fn fs_sync() {
    write_file();
    read_file();
    println!("-----------------------------");
    write_async().await;
    read_async().await;
}

// write something to file sync 
fn write_file() {
    match fs::write("./temp/demo.txt", "hello virtual") {
        Ok(_) => {
            println!("write sync ok")
        },
        Err(e) => {println!("Write sync error: {e}")},
    };
    println!("sync work write:: ->");
}

// read file sync 
fn read_file() {
    match fs::read_to_string("./temp/demo.txt") {
        Ok(content) => {
            println!("sync read file: {content}");
        },
        Err(e) => {println!("read file error: {e}")},
    };
    println!("sync work read:: ->");
}

async fn write_async() {
    match tokio::fs::write("./temp/async_mode.txt", "hello async io").await {
        Ok(_) => {
            println!("write async ok")
        },
        Err(e) => {println!("write async error: {e}")},
    }
    println!("async work write:: ->");
}

async fn read_async() {
    match tokio::fs::read_to_string("./temp/async_mode.txt").await {
        Ok(content) => {
            println!("async read file: {content}")
        },
        Err(e) => {println!("async read file error: {e}")},
    }
    println!("async work read:: ->");
}
