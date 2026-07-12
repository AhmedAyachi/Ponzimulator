use std::collections::VecDeque;
use crate::Resources::{self, User};


pub fn list(_:&mut VecDeque<String>){
    let users=Resources::fetchUsers();
    println!("{:^22}|{:^12}|{:^20}|{:^21}","Id","Name","Amount","Joined On");
    println!("{:-<22}|{:-<12}|{:-<20}|{:-<21}", "", "", "","");
    for user in &users {
        println!("{:^22}| {:<10} | {:<18} |{:^21}",user.id,user.name,user.amount,User::getJoinDate(user.joinedAt));
        println!("{:-<22}|{:-<12}|{:-<20}|{:-<21}", "", "", "","");
    }
}
