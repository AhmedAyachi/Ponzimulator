use std::{env, fs::{File,remove_file}, io::{Error,ErrorKind, Write}, process::{Child, Command, Stdio}, thread, time::Duration};
use rand::{Rng};
use crate::Resources::{Cache, CashFlow};


const PID_FILENAME:&str="daemon.pid";
const BEAT_INTERVAL:f64=2.0; //in secs
const STDOUT_FILENAME:&str="daemon.out";
const STDERR_FILENAME:&str="daemon.err";

pub struct Daemon {} impl Daemon {

    pub fn work(){
        let mut cycleCount=0;
        let mut accounts=Cache::fetchAccounts();
        loop {
            cycleCount+=1;
            let mut solvents:Vec<_>=accounts.iter_mut().filter(|it|{ it.isSolvent() }).collect();
            for account in &mut solvents {
                let offset=rand::rng().random_range(account.getCota());
                if (offset<0.0)&&(offset.abs()>=account.balance) {
                    account.balance=0.0;
                } else {
                    account.balance+=offset;
                }
            }
            if cycleCount>=10 {
                cycleCount=0;
                let amount=CashFlow::getPotentialEarnings(&solvents);
                if amount>0.0 {
                    let potLoss=amount/(solvents.len() as f64);
                    for account in solvents {
                        account.pot-=potLoss;
                    }
                    Cache::saveEarnedAmount(amount);
                }
            }
            Cache::saveAccounts(&accounts);
            thread::sleep(Duration::from_secs_f64(BEAT_INTERVAL));
        }
    }
    

    pub fn stop()->Result<(),Error>{
        if let Ok(pid)=Daemon::getPid() {
            let _=Command::new("kill").args(["-9",&pid]).status().
            expect("couldn't kill daemon process");
            _=remove_file(Daemon::getStdErrFilePath());
            _=remove_file(Daemon::getStdOutFilePath());
            _=remove_file(Daemon::getPidFilePath());
            return Ok(());
        } else {
            return Err(Error::new(ErrorKind::Other,"Daemon already stopped."));
        }
    }

    pub fn start()->Result<Child,Error>{
        if Daemon::isRunning() {
            return Err(Error::new(ErrorKind::Other,"Daemon already running."));
        } else {
            let stdout=File::create(Daemon::getStdOutFilePath()).expect("Couldn't create daemon stdout file");
            let stderr=File::create(Daemon::getStdErrFilePath()).expect("Couldn't create daemon stderr file");
            let currentExe=env::current_exe().expect("Failed to locate current executable");
            let child=Command::new(currentExe).
                arg("--daemon-worker").
                stdout(Stdio::from(stdout)).
                stderr(Stdio::from(stderr)).
                spawn().expect("Couldn't run daemon")
            ;
            let childId=child.id().to_string();
            let mut pidfile=File::create(Daemon::getPidFilePath()).unwrap();
            pidfile.write(childId.as_bytes()).expect("failed to write daemon.pid");
            return Ok(child);
        }
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

    pub fn getPid()->Result<String,Error>{
        let pid=Cache::read(PID_FILENAME);
        return pid;
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
