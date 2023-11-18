use std::collections::HashMap;
use serde::Serialize;


fn main() {
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
    let log = serde_json::to_string(&customer).unwrap();
    println!("{}", log)
}

#[derive(Serialize)]
struct Session {
    customer: String,
    cart: Vec<String>,
    inventory: HashMap<String, usize>,
    account_type: Account,
    shipping: Address,
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