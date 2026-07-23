use crate::Resources::{Cache, CashFlow, Daemon};


pub fn status(_args:&mut Vec<String>){
    let accounts=Cache::fetchAccounts();
    println!("Daemon: {}",Daemon::getStatus());
    println!("Net Pot: {}",CashFlow::getNetPot(&accounts));
    println!("Total Pot: {}",CashFlow::getTotalPot(&accounts));
    println!("Total Deposit: {}",CashFlow::getTotalDeposit(&accounts));
    println!("Earned Amount: {}",CashFlow::getEarnedAmount());
    println!("Accounts Count: {}",accounts.iter().filter(|it|{ it.isSolvent() }).count());
}
