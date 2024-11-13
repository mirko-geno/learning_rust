use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub struct Trader {
    api: String,
    pub api_key: Weak<String>, 
    model: Model<DataFrame, Out>,
    preds: Option<Out>
}

impl Trader {
    pub fn new(api: String, model:Model<DataFrame, Out>, preds: Option<Out>) -> Trader {
        Trader {
            api,
            model,
            preds,
            api_key: Weak::new()
        }
    }

    fn buy(&self) {}
    fn sell(&self) {}

    fn trade(&self) {
        // Logic to buy or sell
    }
}

pub struct Model<I, O> {
    name: String,
    weights: [f64; 3],
    input: Option<I>,
    output: Option<O>
}

impl <I, O> Model <I,O> {
    pub fn new(name: String, weights: [f64; 3], input: Option<I>, output: Option<O>) -> Model<I, O> {
        Model { name, weights, input, output }
    }

    pub fn train(&self) {}

    pub fn test(&self) {}

    pub fn predict(&self) {}
}


pub struct User {
    api_key: Rc<String>,
    pub stocks: HashMap<String, Trader>
}

impl User {
    pub fn new(api_key: String, stock_map: Option<HashMap<String, Trader>>) -> User {
        match stock_map {
            Some(stocks) => User { api_key: Rc::new(api_key), stocks },
            None => User { api_key: Rc::new(api_key), stocks: HashMap::new() }
        }
    }

    pub fn add_stock(&mut self, name: String, mut trader: Trader) {
        trader.api_key = Rc::downgrade(&self.api_key);
        self.stocks.insert(name, trader);
    }
}

pub struct DataFrame {}
pub struct Out {}