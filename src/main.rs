use futures::executor::block_on;
use async_std::task;

fn main() {
    println!("Result for the Value is : {}", block_on(async_main()));
}

async fn async_main() -> i32{
    let neg = neg_async(1).await;
    println!("{}", neg);
    let negating = task::spawn(neg_async(2)).await;
    println!("{}", negating);
    neg + negating    
}

async fn neg_async(f: i32) -> i32 {
    println!("negating for : {}", f);
    task::sleep(std::time::Duration::from_secs(3)).await;
    println!("Finished sleeping for 3 seconds");
    f * -1
}

