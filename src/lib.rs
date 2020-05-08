use async_std::task;
use std::io;

pub async fn async_maint2() -> isize {
    let input = input_async();
    input.await
}

pub async fn input_async() -> isize {
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

pub mod async_maint1;
pub mod async_main;