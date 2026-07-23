use std::{env};


pub fn version(_args:&Vec<String>){
    let name=env!("CARGO_PKG_NAME");
    let version=env!("CARGO_PKG_VERSION");
    println!("{name} v{version}");
}
