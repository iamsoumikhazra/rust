
// using packagesn in rust
use chrono :: {Utc, Local};
use dotenv::dotenv;
use std::env;

fn main(){
    let utc = Utc::now();
    let local = Local::now();
    println!("utc -> {}\nlocal -> {}",utc,local);
    
    dotenv().ok();
    let redis_var = env::var("REDIS_STRING");
    
    match redis_var {
        Ok(str)=>println!("{}",str),
        Err(_e)=>{println!("Error Reading env file.")}
    }
}