use std::fs::{File,OpenOptions};
use std::io::Write;
use crate::Resources::{self};
use chrono::{DateTime};


const CACHE_PATH:&str="./Cache/";
pub struct Cache {
    
} impl Cache {

    pub fn getPath()->String{
        return CACHE_PATH.to_owned();
    }

    /* pub fn getAccountsPath()->String{
        return CACHE_PATH.to_owned()+"accounts.txt";
    } */

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

    pub fn write(basename:&str,content:&str){
        let path=CACHE_PATH.to_owned()+basename+".txt";
        let mut file=File::create(path).expect("Failed to create file");
        write!(file,"{content}").expect("failed to save account");
    }

    pub fn append(basename:&str,content:String){
        let path=CACHE_PATH.to_owned()+basename+".txt";
        let mut file=OpenOptions::new().
            create(true).write(true).append(true).
            open(path).expect(format!("Failed to open {basename} file").as_str())
        ;
        write!(file,"{content}").expect("failed to save account");
    }
    
    pub fn fetchAccounts()->Vec<Account>{
        let content=Cache::read("accounts.txt").unwrap();
        let accounts:Vec<Account>=content.lines().map(|it| Account::new(it)).collect();
        return accounts;
    }

    pub fn saveAccounts(accounts:&Vec<Account>){
        Cache::write("accounts","");
        for account in accounts {
            let line=String::from(
                account.id.clone()+" "+
                account.owner.as_str()+" "+
                account.balance.to_string().as_str()+" "+
                account.createdAt.to_string().as_str()+" "+
                account.totalPot.to_string().as_str()+"\n"
            );
            Cache::append("accounts",line);
        }
    }
}

#[derive(Debug)]
pub struct Account {
    pub id:String,
    pub owner:String,
    pub balance:f64,
    pub createdAt:u64,
    pub totalPot:f64,
} impl Account {
    pub fn new(data:&str)->Self{
        let details:Vec<&str>=data.split_whitespace().collect();
        return Self {
            id:String::from(details[0]),
            totalPot:details[4].parse::<f64>().unwrap_or(0.0),
            owner:String::from(details[1]),
            balance:details[2].parse::<f64>().unwrap_or(0.0),
            createdAt:details[3].parse::<u64>().unwrap_or(0),
        }
    }
    pub fn randomId()->String{
        return Resources::randomId(20);
    }
    pub fn getJoinDate(ms:u64)->String{
        let datetime=DateTime::from_timestamp_millis(ms as i64).expect("");
        return datetime.format("%d-%m-%Y %H:%M:%S").to_string();
    }
}
