use crate::Resources::{Cache, CashFlow, Daemon};


pub fn status(_args:&mut Vec<String>){
    let mut accounts=Cache::fetchAccounts();
    let solvents=accounts.iter_mut().filter(|it|{ it.isSolvent() }).collect();
    println!("Daemon: {}",Daemon::getStatus());
    println!("Net Pot: {}",CashFlow::getNetPot(&solvents));
    println!("Total Pot: {}",CashFlow::getTotalPot(&solvents));
    println!("Total Deposit: {}",CashFlow::getTotalDeposit(&solvents));
    println!("Earned Amount: {}",CashFlow::getEarnedAmount());
    println!("Accounts Count: {}",solvents.len());
}
