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
        let totalPot=CashFlow::getTotalPot(accounts);
        return if netPot>=(0.002*totalPot).min(100.0) { 0.05*netPot } else { 0.0 };
    }

    pub fn getNetPot(accounts:&Vec<Account>)->f64{
        let totalPot=CashFlow::getTotalPot(&accounts);
        let currentPot=CashFlow::getCurrentPot(&accounts);
        return totalPot-currentPot;
    }

    pub fn getTotalPot(accounts:&Vec<Account>)->f64{
        return accounts.iter().fold(0.0,|sum,account|{
            return sum+account.pot;
        });
    }

    pub fn getCurrentPot(accounts:&Vec<Account>)->f64{
        return accounts.iter().fold(0.0,|sum,account|{
            return sum+account.balance;
        });
    }

    pub fn getTotalDeposit(accounts:&Vec<Account>)->f64{
        return accounts.iter().fold(0.0,|sum,account|{
            return sum+account.deposit;
        });
    }
}
