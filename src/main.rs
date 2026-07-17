#![allow(non_snake_case)]
use std::env;
use std::io::{self,Write};
use std::process::Command;

use crate::Resources::Cache;
mod Scripts;
mod Resources;


fn main(){
    let args:Vec<String>=env::args().collect();
    if args.contains(&String::from("--daemon-worker")) {
        std::panic::set_hook(Box::new(|_panicInfo|{
            _=Resources::Daemon::stop();
        }));
        Resources::Daemon::work();
    } else {
        Cache::init();
        loop {
            print!("> ");
            io::stdout().flush().expect("flush failed");
            let mut input=String::new();
            _=io::stdin().read_line(&mut input);
            let args=&mut getArgs(input);
            if args.is_empty() {
                println!("type help to check available commands.");
            } else {
                let cmd=args.remove(0);
                match cmd.as_str() {
                    "help" => Scripts::help(args),
                    "list" => Scripts::list(args),
                    "cashin" => Scripts::cashIn(args),
                    "cashout" => Scripts::cashOut(args),
                    "start" => Scripts::start(args),
                    "stop" => Scripts::stop(args),
                    "status" => Scripts::status(args),
                    "select" => Scripts::select(args),
                    "exit" => std::process::exit(0),
                    "clear" => _=Command::new("clear").status().expect("failed to clear"),
                    _=>{
                        println!("unknown command.");
                        println!("type help to check available commands.");
                    },
                }
            }
        }
    }
}

fn getArgs(input:String)->Vec<String>{
    let input=input.trim().to_string();
    let args=input.split_whitespace().map(|it| it.to_string()).collect();
    return args;
}
