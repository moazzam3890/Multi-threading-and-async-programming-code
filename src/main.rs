use futures::executor::block_on;
use async_std::task;
use std::thread;
use std::io;


fn main() {
    let thread2 = thread::spawn(||{
        println!("Thread2 Value is : {}", block_on(async_maint2()));
    });
    
    let thread1 = thread::spawn(||{
        println!("Thread1 Value is : {}", block_on(async_maint1()));
    });
    
    println!("Result for the main thread Value is : {}", block_on(async_main()));
    thread1.join().unwrap();
    thread2.join().unwrap();
}

async fn async_maint2() -> isize {
    let input = input_async();
    input.await
}

async fn input_async() -> isize {
    println!("Please input your Intiger\n");
    let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    println!("negating for : {}", guess);
    task::sleep(std::time::Duration::from_secs(3)).await;
    println!("Finished sleeping for 3 seconds for {}", guess);
    let returned : isize = guess.trim().parse().expect("Not a Valid Number");
    returned * -1
}
async fn async_maint1() -> i32 {
    let neg1 = neg_async(1);
    // task::sleep(std::time::Duration::from_secs(1)).await;
    println!("Processing the neg1 of thread 1 variable");
    let neg2 = task::spawn(neg_async(2));
    // task::sleep(std::time::Duration::from_secs(1)).await;
    println!("Processing the neg2 of thread 1 Variable with task spawn");
    let neg3 = neg_async(3);
    // task::sleep(std::time::Duration::from_secs(1)).await;
    println!("Processing the neg3 of thread 1 variable");
    let neg4 = neg_async(4);
    // task::sleep(std::time::Duration::from_secs(1)).await;
    println!("Processing the neg4 of thread 1 variable");
    let neg5 = task::spawn(neg_async(5));
    // task::sleep(std::time::Duration::from_secs(1)).await;
    println!("Processing the neg5 of thread 1 variable with the task::spawn");
    // task::sleep(std::time::Duration::from_secs(1)).await;
    neg1.await + neg2.await + neg3.await + neg4.await + neg5.await
}

async fn async_main() -> i32{
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
