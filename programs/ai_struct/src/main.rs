use ai_struct::{User, Trader, Model, DataFrame, Out};

fn main() {
    let amd_df = DataFrame {};
    let nvda_df = DataFrame {};
    let nvda_model = Model::new(String::from("AMD-Stock_AI"), [0.23, 0.81, 2.29], Some(nvda_df), None);
    let amd_model = Model::new(String::from("NVDA-Stock_AI"), [0.10, 0.11, 1.01], Some(amd_df), None);
    let mut mirko = User::new(String::from("24EKja3452F"), None);
    mirko.add_stock(
        String::from("NVDA"),
        Trader::new(String::from("Https:://API-URL.com"), nvda_model, None)
    );

    mirko.add_stock(
        String::from("AMD"),
        Trader::new(String::from("Https:://ANOTHER-API.com"), amd_model, None)
    );

    println!("User API key accessed from nvidia trader instance: {:?}", mirko.stocks["NVDA"].api_key.upgrade());
    println!("User API key accessed from amd trader instance: {:?}", mirko.stocks["AMD"].api_key.upgrade());



}
