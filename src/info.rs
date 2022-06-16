use near_sdk::{env, log};
use std::collections::HashMap;

pub fn cab_fares() -> HashMap<String, f32> {
    HashMap::from([
    ("meme".to_string(),1.0),
    (stg("keke"),2.0),
    (stg("kiki"), 3.0),

    (stg("minto"),4.2),
    (stg("bere"), 5.0),
    (stg("onga"),6.1),

    ])
}

fn stg(destination: &str) -> String {
    String::from(destination)
}

pub fn help() {
    log!("
        Welcome to DriveInco cab services
    ");
    env::log_str("	
        Avalaible functions are:	
        welcome: 'welcome'	
        destinations: 'destinations'	
        order: 'order {\"cab_number\", \"driver_choice\"}'	
        charges: 'charges {\"cab_number\"}'	
        payement: 'payement {\"cab_number\"}'	
        ");	
}

pub fn destinations() {
    env::log_str("\n
            Nairobi
                meme                1.0 Ⓝ, 
                keke                2.0 Ⓝ, 
                kiki                3.0 Ⓝ, 
            Kisumu
                minto               4.2 Ⓝ, 
                bere                5.0 Ⓝ, 
                onga                6.1 Ⓝ,  
        ")
  }
