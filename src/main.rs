#![allow(non_snake_case)]
use std::env;
use std::io::{self,Write};
use std::process::Command;
mod Scripts;
mod Resources;


//#[tokio::main]
fn main(){
    //let mut cmdHistory:Vec<String>=Vec::new();
    let args:Vec<String>=env::args().collect();
    if args.contains(&String::from("--daemon-worker")) {
        Resources::Daemon::work();
    } else {
        loop {
            print!("> ");
            io::stdout().flush().expect("flush failed");
            let mut input=String::new();
            _=io::stdin().read_line(&mut input);
            input=input.trim().to_string();
            let args:&mut Vec<String>=&mut (input.split_whitespace().map(|it| it.to_string()).collect());
            let cmd=args.remove(0);
            //cmdHistory.push(cmd.clone());
            match cmd.as_str() {
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
                    println!("unknown command");
                },
            }
        }
    }
}
