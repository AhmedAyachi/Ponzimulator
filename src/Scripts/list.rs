use std::collections::VecDeque;
use crate::Resources;


pub fn list(args:&mut VecDeque<String>){
    let users=Resources::fetchUsers();
    println!("{:?}",users);
}
