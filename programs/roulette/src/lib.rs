use rand::Rng;
const PROB_ROJO: f64 = 18.0/37.0;

pub struct User {
    balance: f64,
}

impl User {
    pub fn new(balance: f64) -> User {
        User { balance }
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    pub fn timbear(&mut self, mut bet: f64, cap: i32) {
        let mut rng = rand::thread_rng();
        let mut success = rng.gen_bool(PROB_ROJO);

        let mut _counter = 1;
        let profit = bet * 2.0;
        let mut register = bet;

        while !success && self.balance - register > bet*2.0 && _counter <= cap {
            _counter += 1;
            bet *= 2.0;
            register += bet;
            success = rng.gen_bool(PROB_ROJO);
        }

        if success {
            println!("Success in {} times!", _counter);
            self.balance += profit;
        } else if _counter > cap {
            println!("Capped!");
        } else {
            self.balance -= register;
            println!("Not enough founds (${}) to pay the next bet (${})", self.balance, bet*2.0);
            println!("Counter: {_counter}");
        }

        println!("Probability of event was {}%", PROB_ROJO.powf(_counter.into())*100.0);
        println!("Highest bet: {bet}");
    }
}