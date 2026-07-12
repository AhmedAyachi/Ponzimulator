#![allow(non_snake_case)]
use std::io::{self,Write};
use std::collections::VecDeque;
mod Scripts;
mod Resources;


//#[tokio::main]
fn main(){
    loop {
        print!("> ");
        io::stdout().flush().expect("flush failed");
        let mut input=String::new();
        _=io::stdin().read_line(&mut input);
        input=input.trim().to_string();
        let mut chunks:VecDeque<String>=input.split_whitespace().map(|it| it.to_string()).collect();
        let args=&mut chunks;
        let cmd=args.pop_front().unwrap_or_default();
        match cmd.as_str() {
            "list"=>Scripts::list(args),
            "cashin"=>Scripts::cashIn(args),
            "exit"=>std::process::exit(0),
            _=>{
                println!("unknown command");
            },
        }
    }
}
