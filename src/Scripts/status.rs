use std::collections::VecDeque;
use crate::Resources::{Cache};


pub fn status(_args:&mut VecDeque<String>){
    let accounts=Cache::fetchAccounts();
    let income=accounts.iter().fold(0.0,|sum,account|{
        return sum+account.totalPot-account.balance;
    });
    println!("daemon: stopped");
    println!("income: {}",income);
    println!("accounts count: {}",accounts.len());
}
