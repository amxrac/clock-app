use std::time::Duration;
use std::thread;
use std::io;
use std::sync::{mpsc, Arc, Mutex};  

fn main() {
    stopwatch();
}

// fn get_input() {
//     println!("enter current for current time, stopwatch for stopwatch");
// // match user in
// }

fn stopwatch() {  
    struct SharedStates {
        current_time: u32,
        running: bool,
        exit_flag: bool
    }

    enum Command {
        Stop,
        Start,
        Restart,
        Clear
    }

    let stopwatch_duration: u32 = loop {
        println!("enter the time you want to elapse");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        
        match input.trim().parse() {
            Ok(num) if num > 0 => break num,
            _ => println!("enter a valid positive number")
        };
    };

    let (tx, rx) = mpsc::channel();
    
    let states = Arc::new(Mutex::new(SharedStates {
        current_time: 0,
        running: true,
        exit_flag: false
    }));
    
    thread::spawn(move || loop {
        let mut input = String::new();  
        std::io::stdin().read_line(&mut input).unwrap();  
        match input.trim() {
            "stop" => tx.send(Command::Stop).unwrap(),
            "start" => tx.send(Command::Start).unwrap(),
            "restart" => tx.send(Command::Restart).unwrap(),
            "clear" => tx.send(Command::Clear).unwrap(),
            _ => println!("unknown command")         
        }
    });

    let mut was_running = true;

    loop {
        {
            let locked_state = states.lock().unwrap();
            if locked_state.exit_flag {
                println!("you have exited the program succesfully");
                break;
            }
        }        

        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(cmd) => {
                let mut locked_state = states.lock().unwrap();
                match cmd {
                    Command::Stop => locked_state.running = false,
                    Command::Start => locked_state.running = true,
                    Command::Restart => {
                        locked_state.current_time = 0;
                        locked_state.running = true;
                    },

                    Command::Clear => {
                        locked_state.running = false;
                        locked_state.current_time = 0;
                        locked_state.exit_flag = true;
                    },
                }
            },
            Err(mpsc::RecvTimeoutError::Timeout) => {},
            Err(e) => {
                println!("error recieving command {:?}", e);
                break;
            }
        }

        let mut locked_state = states.lock().unwrap();

        if locked_state.running {
            println!("elapsed time: {}", locked_state.current_time);
            thread::sleep(Duration::from_secs(1));
            locked_state.current_time += 1;
            was_running = true;
        }
        else {
            if was_running {
                println!("paused at {}", locked_state.current_time);            
                was_running = false;
            }
        }

        if locked_state.current_time >= stopwatch_duration {
            println!("stopwatch finished successfully");
            break;
        }
    }
}