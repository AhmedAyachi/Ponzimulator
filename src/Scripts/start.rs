use crate::Resources::{Daemon};


pub fn start(_args:&mut Vec<String>){
    if !Daemon::isRunning() {
        println!("Starting daemon...");
    }
    match Daemon::start() {
        Ok(process)=>{
            println!("Daemon started successfully.");
            println!("Daemon process id: {}",process.id());
        },
        Err(error)=>{
            println!("{}",error);
        },
    }
}
