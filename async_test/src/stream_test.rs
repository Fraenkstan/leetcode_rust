use futures::{stream, StreamExt};
use rand::{thread_rng, Rng};
use std::time::Duration;

async fn compute_job(job: i64) -> i64 {
    let mut rng = thread_rng();
    let sleep_ms: u64 = rng.gen_range(0..10);
    tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
    job * job
}

async fn process_result(result: i64) {
    println!("{}", result);
}

pub async fn test1() {
    println!("🤡🤡🤡🤡test1 begin🤡🤡🤡🤡");
    let jobs = 0..100;
    let concurrency = 42;
    stream::iter(jobs)
        .for_each_concurrent(concurrency, |job| async move {
            let result = compute_job(job).await;
            process_result(result).await;
        })
        .await;
    println!("🤡🤡🤡🤡test1 end🤡🤡🤡🤡");
}

pub async fn test2() {
    println!("🐼🐼🐼🐼test2 begin🐼🐼🐼🐼");
    let jobs = 0..100;
    let concurrency = 42;
    stream::iter(jobs)
        .map(compute_job)
        .buffer_unordered(concurrency)
        .for_each(process_result)
        .await;
    println!("🐼🐼🐼🐼test2 end🐼🐼🐼🐼");
}

pub async fn test3() {
    println!("🤖🤖🤖🤖test3 begin🤖🤖🤖🤖");
    let jobs = 0..100;
    let concurrency = 42;
    let result = stream::iter(jobs)
        .map(compute_job)
        .buffer_unordered(concurrency)
        .collect::<Vec<i64>>()
        .await;
    println!("{:?}", result);
    println!("🤖🤖🤖🤖test3 end🤖🤖🤖🤖");
}