use std::{fs::remove_file, process::Command, thread, time::Duration};
use crate::Resources::Cache;


const PID_FILENAME:&str="daemon.pid";
const STDOUT_FILENAME:&str="daemon.out";
const STDERR_FILENAME:&str="daemon.err";

pub struct Daemon {
} impl Daemon {

    pub fn start(){
        loop {
            thread::sleep(Duration::from_secs(5));
        }
    }

    pub fn stop(){
        if let Ok(pid)=Cache::read(PID_FILENAME) {
            println!("stopping process {pid}...");
            let _=Command::new("kill").args(["-9",&pid]).status().
            expect("couldn't kill process");
            _=remove_file(Daemon::getStdErrFilePath());
            _=remove_file(Daemon::getStdOutFilePath());
            _=remove_file(Daemon::getPidFilePath());
            println!("Daemon stopped.");
        };
    }

    pub fn getStatus()->String{
        return match Daemon::isRunning() {
            true => String::from("running"),
            false => String::from("stopped"),
        };
    }

    pub fn isRunning()->bool{
        let pidfile=Cache::read(PID_FILENAME);
        return match pidfile {
            Ok(_)=>true,
            Err(_)=>false,
        };
    }

    pub fn getPidFilePath()->String{
        return Cache::getPath()+"/"+PID_FILENAME;
    }

    pub fn getStdOutFilePath()->String{
        return Cache::getPath()+"/"+STDOUT_FILENAME;
    }

    pub fn getStdErrFilePath()->String{
        return Cache::getPath()+"/"+STDERR_FILENAME;
    }
}
