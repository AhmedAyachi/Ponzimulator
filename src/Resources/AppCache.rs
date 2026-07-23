use std::env::home_dir;
use std::fs::{self,File,OpenOptions};
use std::io::{Error, ErrorKind, Write};
use std::ops::RangeInclusive;
use std::time::{Duration,SystemTime,UNIX_EPOCH};
use crate::Resources::{self};
use chrono::{DateTime};


const ACCOUNTS_KEY:&str="accounts.txt";
const EARNINGS_KEY:&str="earnings.txt";
pub struct Cache {} impl Cache {

    pub fn readEarnings()->String{
        return match Cache::read(EARNINGS_KEY) {
            Ok(content)=>{ content },
            Err(_)=>{ String::from("") },
        }
    }

    pub fn writeEarnings(content:String){
        Cache::write(EARNINGS_KEY,&content);
    }

    pub fn saveEarnedAmount(amount:f64){
        _=Cache::append(EARNINGS_KEY,amount.to_string()+"\n");
    }

    pub fn fetchAccounts()->Vec<Account>{
        return match Cache::read(ACCOUNTS_KEY) {
            Ok(content)=>{ content.lines().map(|it| Account::new(it)).collect() },
            Err(_)=>{ Vec::new() },
        }
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
                account.deposit.to_string().as_str()+" "+
                account.lastDepositAt.to_string().as_str()+"\n",
            );
            _=Cache::append(ACCOUNTS_KEY,line);
        }
    }

    pub fn createAccount(data:Vec<String>)->Result<Account,Error>{
        let owner=data[0].clone();
        let amount=data[1].clone();
        let accountId=Account::randomId();
        let creationTime=SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_millis(0)).as_millis().to_string();
        let data=vec![
            accountId.clone(),
            owner.clone(),
            amount.clone(),
            creationTime.clone(),
            amount.clone(),
            amount.clone(),
            creationTime.clone(),
        ].join(" ");
        if Cache::append(ACCOUNTS_KEY,data.clone()).is_ok() {
            let balance=amount.parse::<f64>().unwrap();
            let createdAt=creationTime.parse::<u64>().unwrap();
            return Ok(Account { 
                id:accountId,
                owner,balance,
                createdAt, 
                pot:balance,
                deposit:balance,
                lastDepositAt:createdAt,
            }); 
        } else {
            return Err(Error::new(ErrorKind::Other,format!("couldn't create account with args {}",data)));
        }
    }

    pub fn read(fileName:&str)->Result<String,Error>{
        let path=Cache::getPath()+fileName;
        if std::fs::exists(&path).unwrap_or(false) {
            let content=std::fs::read_to_string(path).
            expect(format!("Failed to read {fileName} from cache").as_str());
            return Ok(content);
        } else { 
            return Err(Error::new(ErrorKind::Other,format!("No cached file with name: {fileName}")));
        };
    }

    pub fn write(fileName:&str,content:&str){
        let path=Cache::getPath()+fileName;
        let mut file=File::create(path).expect("Failed to create file");
        write!(file,"{content}").expect("failed to save account");
    }

    pub fn append(fileName:&str,content:String)->Result<(),Error>{
        let path=Cache::getPath()+fileName;
        if let Ok(mut file)=OpenOptions::new().
            create(true).write(true).append(true).
        open(path){
            write!(file,"{content}").expect("failed to save account");
            return Ok(());
        } else {
            return Err(Error::new(ErrorKind::Other,format!("Failed to append content to {fileName}.")));
        }
    }

    pub fn init(){
        let path=Cache::getPath();
        if let Ok(exists)=fs::exists(&path) {
            if !exists {
                if let Err(error)=fs::create_dir(&path) {
                    panic!("{}",error);
                }
            }
        } else {
            panic!("couldn't create app cache");
        }
    }

    pub fn getPath()->String{
        return if cfg!(debug_assertions){ String::from("./Cache/") } 
        else { home_dir().unwrap().to_string_lossy().into_owned()+"/.ponzimulator/" };
    }
}


const LOWER_START_COTA:f64=-0.022;
const UPPER_START_COTA:f64=0.03;
const LOWER_END_COTA:f64=-0.0265;
const UPPER_END_COTA:f64=0.015;
/**
 * A day in ms.
 * An account will start losing after 24-25 days.
 * Lowering this value will quicken Cota Shifting.
 */
const MS_STEP:u64=86400000;

pub struct Account {
    pub id:String,
    pub owner:String,
    pub balance:f64,
    pub createdAt:u64,
    pub pot:f64,
    pub deposit:f64,
    pub lastDepositAt:u64,
} impl Account {
    pub fn new(data:&str)->Self{
        let details:Vec<&str>=data.split_whitespace().collect();
        let createdAt=details[3].parse::<u64>().unwrap_or(0);
        let lastDepositAt=match details.get(6) {
            Some(value)=>{ value.parse::<u64>().unwrap_or(createdAt) },
            None=>{ createdAt },
        };
        return Self {
            id:String::from(details[0]),
            owner:String::from(details[1]),
            balance:details[2].parse::<f64>().unwrap_or(0.0),
            createdAt,
            pot:details[4].parse::<f64>().unwrap_or(0.0),
            deposit:details[5].parse::<f64>().unwrap_or(0.0),
            lastDepositAt,
        }
    }

    pub fn isSolvent(&self)->bool{
        return (
            self.pot>0.0 &&
            self.balance>0.0 &&
            self.deposit>0.0
        )
    }
    pub fn isBankrupt(&self)->bool{
        return (
            self.pot>0.0 &&
            self.balance<=0.0 &&
            self.deposit>0.0
        )
    }
    pub fn isCashedOut(&self)->bool{
        return (
            self.pot<=0.0 &&
            self.balance<=0.0 &&
            self.deposit<=0.0
        )
    }

    pub fn toCashedOut(&mut self)->f64{
        let Account {balance,..}=(*self);
        self.pot=0.0;
        self.balance=0.0;
        self.deposit=0.0;
        return balance;
    }
    
    pub fn transact(&mut self,amount:f64)->Result<(),Error>{
        let Account {balance,..}=*self;
        if amount<0.0 && amount.abs()>balance {
            return Err(Error::new(ErrorKind::Other,format!("Insufficient balance: {balance}")));
        } else {
            self.pot+=amount;
            self.deposit+=amount;
            self.balance+=amount;
            if amount>0.0 {
                self.lastDepositAt=SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_millis(0)).as_millis() as u64;
            }
            return Ok(());
        }
    }

    pub fn getCota(&self)->RangeInclusive<f64>{
        let lowerCoef=Account::getCoef(self.getLifetime());
        let upperCoef=Account::getCoef(self.getLastDepositTimestamp());
        let min=LOWER_START_COTA+lowerCoef*(LOWER_END_COTA-LOWER_START_COTA);
        let max=UPPER_START_COTA+upperCoef*(UPPER_END_COTA-UPPER_START_COTA);
        let markup=0.001*self.balance.min(self.deposit);
        return min*markup..=max*markup;
    }

    pub fn getLifetime(&self)->f64{
        let now=SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_millis(0)).as_millis() as u64;
        let lifetime=((now-self.createdAt) as f64)/(MS_STEP as f64);
        return lifetime;
    }

    pub fn getLastDepositTimestamp(&self)->f64{
        let now=SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_millis(0)).as_millis() as u64;
        let lastDepositTime=(now-self.lastDepositAt) as f64/(MS_STEP as f64);
        return lastDepositTime;
    }

    pub fn randomId()->String{
        return Resources::randomId(20);
    }
    pub fn getDate(ms:u64)->String{
        let datetime=DateTime::from_timestamp_millis(ms as i64).expect("");
        return datetime.format("%d/%m/%Y %H:%M:%S").to_string();
    }
    pub fn getCoef(value:f64)->f64{
        //a: -0.137 - c * (-5 - -0.137)
        //b:   26.5 - c * (7 - 26.5)
        return 1.0/(1.0+(-0.137*(value-26.5)).exp());
    }
}
