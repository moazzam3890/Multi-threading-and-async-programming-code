use async_std::task;

pub async fn async_main() -> i32{
    let neg1 = neg_async(1);
    // task::sleep(std::time::Duration::from_secs(2)).await;
    println!("Processing the neg1 variable");
    let neg2 = task::spawn(neg_async(2));
    // task::sleep(std::time::Duration::from_secs(2)).await;
    println!("Processing the neg2 Variable with task::spawn");
    let neg3 = neg_async(3);
    // task::sleep(std::time::Duration::from_secs(2)).await;
    println!("Processing the neg3 variable");
    let neg4 = neg_async(4);
    // task::sleep(std::time::Duration::from_secs(2)).await;
    println!("Processing the neg4 variable");
    let neg5 = task::spawn(neg_async(5));
    // task::sleep(std::time::Duration::from_secs(2)).await;
    println!("Processing the neg5 variable with the task::spawn");
    // task::sleep(std::time::Duration::from_secs(2)).await;
    neg1.await + neg2.await + neg3.await + neg4.await + neg5.await  
}

async fn neg_async(f: i32) -> i32 {
    println!("negating for : {}", f);
    task::sleep(std::time::Duration::from_secs(3)).await;
    println!("Finished sleeping for 3 seconds for {}", f);
    f * -1
}
