use chrono::prelude::*;
use std::time::Duration;
use std::thread;
use std::io;

fn main() {
    // let current_time: DateTime<Local> = Local::now();
    // println!("time: {:?}", current_time);
    stopwatch();
}

// fn get_input() {
//     println!("enter 1 for current time, 2 for stopwatch");
// // match user in
// }

fn stopwatch() {
    let user_input: i32 = loop {
        println!("enter the time you want to elapse");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        
        let input: i32 = match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => { 
                println!("enter a valid number");
                continue;
            }
        };
    };

    let value = user_input + 1;

    for i in (1..value).rev() {
        println!("{:?}", i - 1);
        thread::sleep(Duration::from_secs(1));
    };

}