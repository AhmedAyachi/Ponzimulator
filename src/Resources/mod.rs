mod fetchUsers;
use rand;


pub use fetchUsers::*;


//let ranges
pub fn randomId(length:u16)->String{
    let mut id=String::new();
    for _ in 0..length {
        let range=rand::random_range(0..3);
        let c:char=match range {
            0=>rand::random_range(48..58) as u8 as char,
            1=>rand::random_range(65..91) as u8 as char,
            2=>rand::random_range(97..122) as u8 as char,
            _=>'-',
        };
        id=id+c.to_string().as_str();
    };
    return id;
}

const CACHE_PATH:&str="./Cache/";
pub fn readCache(basename:&str)->String{
    let content=std::fs::read_to_string(CACHE_PATH.to_owned()+basename+".txt").expect("zadazd");
    return content;
}
