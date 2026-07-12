#![allow(non_snake_case)]
use std::io;
use std::fs::File;
use std::time::Duration;
use daemonize::{Daemonize,Error};


pub fn startDaemon<Callback,Fallback>(callback:Callback,fallback:Fallback) where 
    Callback:FnOnce()+Send+'static,
    Fallback:FnOnce(Error)+Send+'static,
{
    let stdout=File::create("./service.out").unwrap();
    let stderr=File::create("./service.err").unwrap();
    let daemon=Daemonize::new().
        pid_file("./service.pid").
        working_directory("./").
        stdout(stdout).stderr(stderr)
    ;
    println!("starting daemon...");
    match daemon.start() {
        Ok(_)=>{
            callback();
        },
        Err(error)=>{
            fallback(error);
        },
    }
}
