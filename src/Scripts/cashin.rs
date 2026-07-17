
use crate::Resources::{Cache, Daemon};


pub fn cashIn(args:&mut Vec<String>){
    if areValidArgs(args) {
        let shouldRerun=Daemon::isRunning();
        if shouldRerun { Daemon::stop().unwrap() };
        if args.len()==2 {//name amount //adds a new account
            if let Ok(account)=Cache::createAccount(args.clone()) {
                println!("owner: {}",account.owner);
                println!("account id: {}",account.id);
            } else {
                println!("Failed to create new account");
            }
        }
        else if args.len()==3 {//name accountId amount //adds amount to an existant account
            let owner=args[0].clone();
            let accountId=args[1].clone();
            let balance=args[2].clone();
            let mut accounts=Cache::fetchAccounts();
            if let Some(account)=accounts.iter_mut().find(|account| (account.id==accountId)&&(account.owner==owner)) {
                let amount=balance.parse::<f64>().unwrap_or(0.0);
                match account.transact(amount) {
                    Ok(_)=>{ 
                        Cache::saveAccounts(&accounts);
                        println!("✔ {amount} has been deposited in {owner}'s account {accountId}.");
                    },
                    Err(error)=>{ println!("{}",error) },
                };
            } else {
                println!("no account found.");
            }
        }
        if shouldRerun { _=Daemon::start() };
    } else {
        println!("Invalid usage: cashin accepts either 2 or 3 args.");
        println!("owner amount #to create a new account");
        println!("owner accountId amount #to add balance to an account");
    }
}

fn areValidArgs(args:&Vec<String>)->bool{
    if args.len()==2 { return args[1].parse::<f64>().is_ok() }
    else if args.len()==3 { return args[2].parse::<f64>().is_ok() }
    else { return false };
}
