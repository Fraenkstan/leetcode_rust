use crate::stream_test::{test1, test2, test3};

mod stream_test;

#[tokio::main]
async fn main() {
    test1().await;
    test2().await;
    test3().await;
}
