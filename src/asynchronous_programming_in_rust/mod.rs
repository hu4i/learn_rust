use chrono::Timelike;

pub mod timer_future;
pub mod executor;
pub mod pinning;

pub fn dbgprint(function: &str, dscpt: &str){
    use std::thread;
    use chrono::{DateTime, Local};

    let now: DateTime<Local> = Local::now();

    println!("[*] [{:02}:{:02}:{:02}] [{:?}] [{function}] {dscpt}", 
        now.hour(),
        now.minute(), 
        now.second(),
        thread::current().id());
}