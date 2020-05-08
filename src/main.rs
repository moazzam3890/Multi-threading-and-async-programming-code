use futures::executor::block_on;
use std::thread;
use input::async_maint2;
use input::async_maint1::async_maint1;
use input::async_main::async_main;



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
// mod returning_to_main_async_main_2;



