use std::fmt::Error;
use std::fs::{File,OpenOptions};
use std::io::Write;
use std::time::{Duration,SystemTime,UNIX_EPOCH};
use crate::Resources::{self};
use chrono::{DateTime};


const CACHE_PATH:&str=if cfg!(debug_assertions){ "./Cache/" } else { "/.ponzimulator/" };
const ACCOUNTS_KEY:&str="accounts.txt";
const EARNINGS_KEY:&str="earnings.txt";
pub struct Cache {
    
} impl Cache {

    pub fn readEarnings()->String{
        let content=Cache::read(EARNINGS_KEY).expect(format!("Failed to read {EARNINGS_KEY}").as_str());
        return content;
    }

    pub fn writeEarnings(content:String){
        Cache::write(EARNINGS_KEY,&content);
    }

    pub fn saveEarnedAmount(amount:f64){
        Cache::append(EARNINGS_KEY,amount.to_string()+"\n");
    }

    pub fn fetchAccounts()->Vec<Account>{
        let content=Cache::read(ACCOUNTS_KEY).expect(format!("Failed to read {ACCOUNTS_KEY}").as_str());
        let accounts:Vec<Account>=content.lines().map(|it| Account::new(it)).collect();
        return accounts;
    }

    pub fn saveAccounts(accounts:&Vec<Account>){
        Cache::write(ACCOUNTS_KEY,"");
        for account in accounts {
            let line=String::from(
                account.id.clone()+" "+
                account.owner.as_str()+" "+
                account.balance.to_string().as_str()+" "+
                account.createdAt.to_string().as_str()+" "+
                account.pot.to_string().as_str()+" "+
                account.deposit.to_string().as_str()+"\n"
            );
            Cache::append(ACCOUNTS_KEY,line);
        }
    }

    pub fn createAccount(data:Vec<String>)->Result<Account,Error>{
        let owner=data[0].clone();
        let amount=data[1].clone();
        let accountId=Account::randomId();
        let creationTime=SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_millis(0)).as_millis().to_string();
        Cache::append(ACCOUNTS_KEY,vec![
            accountId.clone(),
            owner.clone(),
            amount.clone(),
            creationTime.clone(),
            amount.clone(),
            amount.clone(),
        ].join(" "));
        let balance=amount.parse::<f64>().unwrap();
        return Ok(Account { 
            id:accountId,
            owner,balance,
            createdAt:creationTime.parse::<u64>().unwrap(), 
            pot:balance,
            deposit:balance,
        });
    }

    pub fn read(fileName:&str)->Result<String,String>{
        let path=CACHE_PATH.to_owned()+fileName;
        if std::fs::exists(&path).unwrap_or(false) {
            let content=std::fs::read_to_string(path).
            expect(format!("Failed to read {fileName} from cache").as_str());
            return Ok(content);
        } else { 
            return Err(String::from("No cached file with name: ")+fileName);
        };
    }

    pub fn write(fileName:&str,content:&str){
        let path=CACHE_PATH.to_owned()+fileName;
        let mut file=File::create(path).expect("Failed to create file");
        write!(file,"{content}").expect("failed to save account");
    }

    pub fn append(fileName:&str,content:String){
        let path=CACHE_PATH.to_owned()+fileName;
        let mut file=OpenOptions::new().
            create(true).write(true).append(true).
            open(path).expect(format!("Failed to open {fileName} file").as_str())
        ;
        write!(file,"{content}").expect("failed to save account");
    }

    pub fn getPath()->String{
        return CACHE_PATH.to_owned();
    }
}

#[derive(Debug)]
pub struct Account {
    pub id:String,
    pub owner:String,
    pub balance:f64,
    pub createdAt:u64,
    pub pot:f64,
    pub deposit:f64,
} impl Account {
    pub fn new(data:&str)->Self{
        let details:Vec<&str>=data.split_whitespace().collect();
        return Self {
            id:String::from(details[0]),
            owner:String::from(details[1]),
            balance:details[2].parse::<f64>().unwrap_or(0.0),
            createdAt:details[3].parse::<u64>().unwrap_or(0),
            pot:details[4].parse::<f64>().unwrap_or(0.0),
            deposit:details[5].parse::<f64>().unwrap_or(0.0),
        }
    }
    
    pub fn depositAmount(&mut self,amount:f64){
        self.pot+=amount;
        self.deposit+=amount;
        self.balance+=amount;
    }

    pub fn randomId()->String{
        return Resources::randomId(20);
    }
    pub fn getCreationDate(ms:u64)->String{
        let datetime=DateTime::from_timestamp_millis(ms as i64).expect("");
        return datetime.format("%d/%m/%Y %H:%M:%S").to_string();
    }
}
