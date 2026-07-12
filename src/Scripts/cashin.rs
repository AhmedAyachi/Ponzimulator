use std::collections::VecDeque;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{Duration,SystemTime, UNIX_EPOCH};
use crate::Resources;



pub fn cashIn(args:&mut VecDeque<String>){
    if args.len()>1 {
        if args[1].parse::<f64>().is_ok() {
            let mut usersFile=OpenOptions::new().
                create(true).write(true).append(true).
                open("./Cache/users.txt").
                expect("Failed to open users file");
            args.push_back(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_millis(0)).as_millis().to_string());
            args.push_front(Resources::User::randomId());
            let buffer=args.make_contiguous().join(" ");
            writeln!(usersFile,"{buffer}").expect("failed to save user");
        } else {
            println!("second arg must be a number");
        }
    } else {
        println!("cashin requires at least 2 args: name amount");
    }
}
