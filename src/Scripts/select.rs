use crate::Resources::{Account, Cache};


pub fn select(args:&mut Vec<String>){
    if args.len()==1 {
        let accountId=&args[0];
        let accounts=Cache::fetchAccounts();
        if let Some(account)=accounts.iter().find(|account|{ account.id==(*accountId) }) {
            println!("Owner: {}",account.owner);
            if cfg!(debug_assertions) {
                println!("Cota: {:?}",account.getCota());
                println!("Lifetime: {}",account.getLifetime());
            }
            println!("Deposit: {}",account.deposit);
            println!("Balance: {}",account.balance);
            println!("Created on: {}",Account::getDate(account.createdAt));
            println!("Last deposit at: {}",Account::getDate(account.lastDepositAt));
        } else {
            println!("no such account.");
        }
    } else {
        println!("select requires an account id as argument.");
    }
}
