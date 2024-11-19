use rand::Rng;

const PROB_ROJO: f64 = 18.0/37.0;

struct User {
    balance: f64,
    bet: f64,
    spent_money: f64,
    counter: u32,
    returned: f64
}

impl User {
    fn new(balance: f64, bet: f64) -> User {
        User {
            balance,
            bet,
            spent_money: 0.0,
            counter: 0,
            returned: 0.0,
        }
    }
    fn timbear(&mut self) {
        let mut rng = rand::thread_rng();
        let mut success = false;

        while !success {
            self.counter += 1;
            self.bet *= 2.0;
            self.spent_money += self.bet;
            success = rng.gen_bool(PROB_ROJO);
        }
        println!("Success in {} times!", self.counter);
        println!("Probability of event was {}%", PROB_ROJO.powf(self.counter.into(f64))*100);
        println!("Highest bet: {bet},   Spent money {spent_money}");
    
    }
}
fn main() {
    let user = User::new(10);

}