use crate::Resources::{Cache, Daemon};



pub fn cashOut(args:&mut Vec<String>){
    let argCount=args.len();
    if (argCount==2)||(argCount==3) {
        let shouldRerun=Daemon::isRunning();
        if shouldRerun { _=Daemon::stop() };
        let owner=args.get(0).unwrap();
        let accountId=args.get(1).unwrap();
        let mut accounts=Cache::fetchAccounts();
        if let Some(index)=accounts.iter().position(|account|{ 
            return (account.id==(*accountId))&&(account.owner==(*owner)); 
        }){
            if let Some(arg2)=args.get(2) {
                if let Ok(amount)=arg2.parse::<f64>() {
                    let account=&mut accounts[index];
                    match account.transact(-1.0*amount) {
                        Ok(_)=>{
                            Cache::saveAccounts(&accounts);
                            println!("✔ {amount} has been withdrawn from {owner}'s account {accountId}.");
                        },
                        Err(error)=>{ println!("{}",error) },
                    }
                } else {
                    println!("Third argument (amount) must be a number.");
                }
            } else {
                let account=accounts.remove(index);
                Cache::saveAccounts(&accounts);
                println!("{} has been withdrawn from the pot.",account.balance);
                println!("✔ {owner}'s account {accountId} has been cashed out.");
            }
        }
        if shouldRerun { _=Daemon::start() };
    } else {
        println!("Cashout accepts either 2 or 3 arguments.");
        println!("owner accountId #to fully cashout");
        println!("owner accountId amount #to cashout an amount");
    }
}
