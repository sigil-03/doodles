use chrono::{DateTime, Local, SubsecRound, TimeDelta};
use std::{thread, time::Duration};

fn main() {
    let mut time_a = chrono::Local::now();
    loop {
        if (Local::now().signed_duration_since(time_a)) >= TimeDelta::seconds(3) {
            time_a = chrono::Local::now();
            println!("three is a wonderful number, don't you think?");
        }
        thread::sleep(Duration::from_millis(100));        
    }
}
