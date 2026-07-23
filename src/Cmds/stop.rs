use crate::Resources::Daemon;


pub fn stop(_args:&Vec<String>){
    if Daemon::isRunning() {
        println!("stopping process {}...",Daemon::getPid().unwrap());
    }
    match Daemon::stop() {
        Ok(_)=>{
            println!("Daemon stopped.");
        },
        Err(error)=>{
            println!("{}",error);
        },
    }
}
