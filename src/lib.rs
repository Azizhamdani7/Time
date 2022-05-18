use near_sdk::borsh::{self,BorshSerialize,BorshDeserialize};
use near_sdk::{near_bindgen,PanicOnDefault,env};
use chrono::{Datelike, Timelike, Utc, prelude, NaiveDateTime, DateTime};

#[near_bindgen]
#[derive(BorshSerialize,BorshDeserialize,PanicOnDefault)]

struct Time{
    time: u64,
}

#[near_bindgen]
impl Time{

    // Initialization
    #[init]
    pub fn new() -> Self{
        let t = Self{
            time : 1
        };
        t
    }


// In this function we will set the time by using block_timestamp()
    pub fn set_time(&mut self) {
        self.time = env::block_timestamp();
        // let timestamp = self.time;
        
    }

    // In this function we will get the time
    pub fn get_time(&self) -> u64 {
       let timestamp = self.time;
//        let naive = NaiveDateTime::from_timestamp(timestamp.try_into().unwrap(), 0);
//     //    let datetime : DateTime<_>  = DateTime::from_utc(naive, Utc);
//    let datetime = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(600000000, 0), Utc).time();
//        let newdate = datetime.format("%Y-%m-%d %H:%M:%S");
//         println!("{}", newdate);
        timestamp
       
    }

}    