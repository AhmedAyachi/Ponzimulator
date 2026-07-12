use crate::Resources;
use chrono::{DateTime};


pub fn fetchUsers()->Vec<User>{
    let content=Resources::readCache("users");
    let users:Vec<User>=content.lines().map(|it| User::new(it)).collect();
    return users;
}

#[derive(Debug)]
pub struct User {
    pub id:String,
    pub name:String,
    pub amount:f64,
    pub joinedAt:u64,
} 
impl User {
    pub fn new(data:&str)->Self{
        let details:Vec<&str>=data.split_whitespace().collect();
        return Self {
            id:String::from(details[0]),
            name:String::from(details[1]),
            amount:details[2].parse::<f64>().unwrap_or(0.0),
            joinedAt:details[3].parse::<u64>().unwrap_or(0),
        }
    }
    pub fn randomId()->String{
        return Resources::randomId(20);
    }
    pub fn getJoinDate(ms:u64)->String{
        let datetime=DateTime::from_timestamp_millis(ms as i64).expect("");
        return datetime.format("%d-%m-%Y %H:%M:%S").to_string();
    }
}
