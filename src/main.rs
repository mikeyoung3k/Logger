#![allow(dead_code)]

use std::collections::HashMap;
use serde::Serialize;
use std::fmt;

use log::{debug,info,error};
use env_logger::Env;

fn main() {
    let env = Env::default().filter_or("LOG_LEVEL","info");
    env_logger::init_from_env(env);

    let mut inventory = HashMap::new();
    inventory.insert("servers".to_string(),5);
    inventory.insert("screens".to_string(),3);

    let customer = Session{
        customer: "Foo".to_string(),
        cart: vec!["Pliers".to_string(),"Rice".to_string(),"Toothpaste".to_string()],
        inventory: inventory,
        account_type: Account::Corporate("CompuTech".to_string()),
        shipping: Address{
            country: "Austria".to_string(),
            location: ["Mouse".to_string(), "Coffee".to_string(), "Tall".to_string()],
            postcode: "HA34 7ER".to_string(),
        }
    };

    info!("{}",customer);
    debug!("A Debug log: {}",customer);
    error!("An error log: {}",customer);


}

#[derive(Serialize)]
struct Session {
    customer: String,
    cart: Vec<String>,
    inventory: HashMap<String, usize>,
    account_type: Account,
    shipping: Address,
}

impl fmt::Display for Session {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let log = serde_json::to_string(&self);
        match log {
            Ok(l) => write!(f,"{}",l),
            Err(e) => write!(f,"Failed to serialize log struct: {}",e),
        }
    }
}

#[derive(Serialize)]
enum Account {
    Free,
    Premium,
    Corporate(String),
}

#[derive(Serialize)]
struct Address {
    country: String,
    location: [String;3],
    postcode: String,
}