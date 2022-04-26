use chrono::Local;
use std::{
    thread,
    time::{self, Duration},
};
use tokio::{self, runtime::Runtime};

/// 单一线程
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    println!("Server started.");
    thread::sleep(time::Duration::from_secs(10));
    println!("Server exited.");
    multiple_thread();
}

/// 多线程
pub fn multiple_thread() {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            println!("Create a multiple thread runtime.");
        });
}

/// 多个 Runtime 共存
/// - 一个 OS 主线程
/// - 两个子线程
/// - 16 个 Worker 线程
pub fn _multiple_runtime_coexist() {
    let t1 = thread::spawn(|| {
        let _rt = Runtime::new().unwrap();
        thread::sleep(Duration::from_secs(10));
    });

    let t2 = thread::spawn(|| {
        let _rt = Runtime::new().unwrap();
        thread::sleep(Duration::from_secs(10));
    });
    t1.join().unwrap();
    t2.join().unwrap();
}

// 在异步runtime中执行异步任务

pub fn _run_task_in_runtime() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        println!("before sleep: {}", Local::now().format("%F %T.%3f"));

        // 主动释放 CPU, 从新进入调度队列等待下次执行
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        println!("after sleep: {}", Local::now().format("%F %T.%3f"));
    });
}
