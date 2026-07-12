use std::collections::VecDeque;
use crate::Resources::{Account,Cache};


pub fn list(_:&mut VecDeque<String>){
    let accounts=Cache::fetchAccounts();
    println!("{:^22}|{:^12}|{:^20}|{:^21}|{:^20}","Id","Owner","Balance","Created At","Total Pot");
    println!("{:-<22}|{:-<12}|{:-<20}|{:-<21}|{:-<20}","","","","","");
    for account in &accounts {
        println!(
            "{:^22}| {:<10} | {:<18} |{:^21}| {:<18} ",
            account.id,account.owner,account.balance,
            Account::getJoinDate(account.createdAt),
            account.totalPot,
        );
        println!("{:-<22}|{:-<12}|{:-<20}|{:-<21}|{:-<18}","","","","","");
    }
}
