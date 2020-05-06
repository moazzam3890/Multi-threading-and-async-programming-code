use futures::executor::block_on;
use async_std::task;

fn main() {
    result = block_on(async_main());
    println!("{}",result);
}

async fn async_main() -> i32{
    let neg = neg_async(1);
    println!("{}", neg_async());
    let negating = task::spawn(neg_async(2));
    neg.await + negating.await
}

