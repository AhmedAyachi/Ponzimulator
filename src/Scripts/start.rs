use std::{collections::VecDeque, env, fs::File, io::Write, process::{Child,Command,Stdio}};
use crate::Resources::{Daemon};




pub fn start(_args:&mut VecDeque<String>)->Child{
    println!("starting daemon...");
    let stdout=File::create(Daemon::getStdOutFilePath()).unwrap();
    let stderr=File::create(Daemon::getStdErrFilePath()).unwrap();
    let currentExe=env::current_exe().expect("Failed to locate current executable");
    let child=Command::new(currentExe).
        arg("--daemon-worker").
        stdout(Stdio::from(stdout)).
        stderr(Stdio::from(stderr)).
        spawn().expect("msg")
    ;
    let childId=child.id().to_string();
    let mut pidfile=File::create(Daemon::getPidFilePath()).unwrap();
    pidfile.write(childId.as_bytes()).expect("failed to write daemon.pid");
    println!("Daemon process id: {}",childId);
    return child;
}
