use near_sdk::{env, log};
use std::collections::HashMap;

pub fn cab_fares() -> HashMap<String, f32> {
    HashMap::from([
    ("james".to_string(),1.0),
    (stg("mary"),2.0),
    (stg("jane"), 3.0),
    (stg("kiki"),4.2),
    (stg("barry"), 5.0),
    (stg("olga"),6.1),
    (stg("muturi"), 9.0),
    (stg("makori"), 9.5),
    (stg("mesita"),10.0)
    ])
}

fn stg(destination: &str) -> String {
    String::from(destination)
}

pub fn help() {
    log!("
        Welcome to DriveInco Cab Services
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
                james              1.0 Ⓝ, 
                mary               2.0 Ⓝ, 
                jane               3.0 Ⓝ, 
            Kisumu
                kiki               4.2 Ⓝ, 
                barry              5.0 Ⓝ, 
                olga               6.1 Ⓝ,  
            Moyale
                muturi             9.0 Ⓝ, 
                makori             9.5 Ⓝ, 
                mesita             10.0 Ⓝ,
        ")
  }
