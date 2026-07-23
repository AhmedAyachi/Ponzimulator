#![allow(unused_parens)]
#![allow(non_snake_case)]
use std::env;
use std::io::{self,Write};
use std::process::Command;

use crate::Cmds::version;
use crate::Resources::Cache;
mod Cmds;
mod Resources;


fn main(){
    let mut args:Vec<String>=env::args().collect();
    if args.contains(&String::from("--daemon-worker")) {
        std::panic::set_hook(Box::new(|_panicInfo|{
            _=Resources::Daemon::stop();
        }));
        Resources::Daemon::work();
    } else {
        args.remove(0);
        Cache::init();
        if args.len()>0 {
            let cmd=args.remove(0);
            match cmd.as_str() {
                "-v"|"--version" => version(&args),
                _=>{
                    println!("unknown command: {cmd}.");
                    println!("pass no arguments to launch the interactive terminal.");
                },
            }
        } else { loop {
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
                    "help" => Cmds::help(args),
                    "list" => Cmds::list(args),
                    "cashin" => Cmds::cashIn(args),
                    "cashout" => Cmds::cashOut(args),
                    "start" => Cmds::start(args),
                    "stop" => Cmds::stop(args),
                    "status" => Cmds::status(args),
                    "select" => Cmds::select(args),
                    "exit" => std::process::exit(0),
                    "clear" => _=Command::new("clear").status().expect("failed to clear"),
                    _=>{
                        println!("unknown command.");
                        println!("type help to check available commands.");
                    },
                }
            }
        }};
    }
}

fn getArgs(input:String)->Vec<String>{
    let input=input.trim().to_string();
    let args=input.split_whitespace().map(|it| it.to_string()).collect();
    return args;
}
