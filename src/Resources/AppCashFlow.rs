use crate::Resources::{Account, Cache};


pub struct CashFlow {} impl CashFlow {

    pub fn getEarnedAmount()->f64{
        let content=Cache::readEarnings();
        let amount=content.lines().fold(0.0,|sum,line|{
            return sum+line.parse::<f64>().unwrap_or(0.0);
        });
        Cache::writeEarnings(amount.to_string()+"\n");
        return amount;
    }

    pub fn getPotentialEarnings(accounts:&Vec<Account>)->f64{
        let netPot=CashFlow::getNetPot(accounts);
        return if netPot>=100.0 { 0.1*netPot } else { 0.0 };
    }

    pub fn getNetPot(accounts:&Vec<Account>)->f64{
        let totalPot=CashFlow::getTotalPot(&accounts);
        let currentPot=CashFlow::getCurrentPot(&accounts);
        return totalPot-currentPot;
    }

    pub fn getCurrentPot(accounts:&Vec<Account>)->f64{
        return accounts.iter().fold(0.0,|sum,account|{
            return sum+account.balance;
        });
    }

    pub fn getTotalPot(accounts:&Vec<Account>)->f64{
        return accounts.iter().fold(0.0,|sum,account|{
            return sum+account.pot;
        });
    }
}
