use std::collections::VecDeque;
use std::time::{Duration,SystemTime,UNIX_EPOCH};
use crate::Resources::{Account,Cache};



pub fn cashIn(args:&mut VecDeque<String>){
    if args.len()==2 {//name amount
        let amount=args[1].clone();
        if amount.parse::<f64>().is_ok() {
            let owner=args[0].clone();
            let accountId=Account::randomId();
            args.push_front(accountId.clone());
            args.push_back(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_millis(0)).as_millis().to_string());
            args.push_back(amount);
            Cache::append("accounts",args.make_contiguous().join(" "));
            println!("owner: {}",owner);
            println!("account id: {}",accountId);
        } else {
            println!("second arg must be a number");
        }
    }
    else if args.len()==3 {//name accountId amount
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
    } else {
        println!("cashin accepts either 2 or 3 args.");
        println!("owner balance #to create a new account");
        println!("owner accountId balance #to add balance to an account");
    }
}
