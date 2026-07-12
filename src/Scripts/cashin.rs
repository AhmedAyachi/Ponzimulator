use std::collections::VecDeque;
use std::time::{Duration,SystemTime,UNIX_EPOCH};
use crate::Resources::{Account,Cache};



pub fn cashIn(args:&mut VecDeque<String>){
    if args.len()==2 {
        if args[1].parse::<f64>().is_ok() {
            args.push_back(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_millis(0)).as_millis().to_string());
            args.push_front(Account::randomId());
            let buffer=args.make_contiguous().join(" ");
            Cache::append("accounts",buffer);
        } else {
            println!("second arg must be a number");
        }
    }
    else if args.len()==3 {
        let balance=args[2].to_owned();
        if balance.parse::<f64>().is_ok() {
            let owner=args[0].to_owned();
            let accountId=args[1].to_owned();
            let mut accounts=Cache::fetchAccounts();
            if let Some(account)=accounts.iter_mut().find(|account| (account.id==accountId)&&(account.owner==owner)) {
                let amount=balance.parse::<f64>().unwrap_or(0.0);
                account.balance+=amount;
                account.totalPot+=amount;
                Cache::saveAccounts(&accounts);
            } else {
                println!("no account found");
            }
        } else {
            println!("third arg must be a number");
        }
    } 
    else {
        println!("cashin accepts either 2 or 3 args.");
        println!("owner balance #to create a new account");
        println!("owner accountId balance #to add balance to an account");
    }
}
