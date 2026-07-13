use crate::Resources::{Account,Cache};


pub fn list(_:&mut Vec<String>){
    let accounts=Cache::fetchAccounts();
    println!("{:^22}|{:^12}|{:^20}|{:^21}|{:^20}|{:^20}","Id","Owner","Balance","Created At","Pot","Deposit");
    println!("{:-<22}|{:-<12}|{:-<20}|{:-<21}|{:-<20}|{:-<20}","","","","","","");
    for account in &accounts {
        println!(
            "{:^22}| {:<10} | {:<18} |{:^21}| {:<18} | {:<18} ",
            account.id,account.owner,account.balance,
            Account::getCreationDate(account.createdAt),
            account.pot,account.deposit,
        );
        println!("{:-<22}|{:-<12}|{:-<20}|{:-<21}|{:-<20}|{:-<20}","","","","","","");
    }
}
