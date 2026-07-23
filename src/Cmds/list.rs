use crate::Resources::{Account,Cache};


pub fn list(_:&mut Vec<String>){
    let accounts=Cache::fetchAccounts();
    println!(
        "{:^22}|{:^12}|{:^20}|{:^20}|{:^21}",
        "Id","Owner","Balance","Deposit","Created On",
    );
    println!("{:-<22}|{:-<12}|{:-<20}|{:-<20}|{:-<21}","","","","","");
    for account in &accounts {
        if account.isSolvent() {
            println!(
                "{:^22}| {:<10} | {:<18} | {:<18} |{:^21}",
                account.id,account.owner,
                account.balance,account.deposit,
                Account::getDate(account.createdAt),
            );
            println!("{:-<22}|{:-<12}|{:-<20}|{:-<20}|{:-<21}","","","","","");
        }
    }
}
