use rand::Rng;
#[allow(unused_variables)] 
#[allow(dead_code)] 
enum Coin {
    Penny,
    Nickel,
    Dimmel,
    Quarter
}

impl Coin {
    fn in_cents(self) -> u8 {
        match self {
            Coin::Penny => {
                if rand::thread_rng().gen_range(1..=100) <= 5 {println!("Lucky penny");}
                1
            },
            Coin::Nickel => 5,
            Coin::Dimmel => 10,
            Coin::Quarter => 25
        }
    }
}


fn main() {
    let coin: Coin = Coin::Penny;
    println!("{}", coin.in_cents());
}
