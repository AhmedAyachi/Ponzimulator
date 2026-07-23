use crate::Resources::{Account, Cache};


pub fn select(args:&mut Vec<String>){
    if args.len()==1 {
        let accountId=&args[0];
        let accounts=Cache::fetchAccounts();
        if let Some(account)=accounts.iter().find(|account|{ account.id==(*accountId) }) {
            let isSolvent=account.isSolvent();
            println!("Owner: {}",account.owner);
            if isSolvent {
                if cfg!(debug_assertions) {
                    println!("Cota: {:?}",account.getCota());
                    println!("Lifetime: {}",account.getLifetime());
                    println!("Deposit timestamp: {}",account.getLastDepositTimestamp());
                }
                println!("Pot: {}",account.pot);
                println!("Deposit: {}",account.deposit);
                println!("Balance: {}",account.balance);
                
            } else {
                let status:&str=(||{
                    if account.isCashedOut() { return "cashed out" }
                    else if account.isBankrupt() { return "bankrupt" }
                    else { return "unknown" };
                })();
                println!("Status: {status}");
            }
            println!("Created on: {}",Account::getDate(account.createdAt));
            if isSolvent {
                println!("Last deposit at: {}",Account::getDate(account.lastDepositAt));
            }
        } else {
            println!("no such account.");
        }
    } else {
        println!("select requires an account id as argument.");
    }
}
