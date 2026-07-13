use crate::Resources::{Account, Cache};


pub fn select(args:&mut Vec<String>){
    if args.len()==1 {
        let accountId=&args[0];
        let accounts=Cache::fetchAccounts();
        if let Some(account)=accounts.iter().find(|account|{ account.id==(*accountId) }) {
            println!("Owner: {}",account.owner);
            println!("balance: {}",account.balance);
            println!("Created on: {}",Account::getCreationDate(account.createdAt));
        } else {
            println!("no such account.");
        }
    } else {
        println!("select requires an account id as argument.");
    }
}
