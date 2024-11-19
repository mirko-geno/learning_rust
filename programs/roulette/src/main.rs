use roulette::User;


fn main() {
    let mut user = User::new(10000.0);

    let bet = user.get_balance()/100.0;
    let cap = 4;
    for _i in 0..10 {
        user.capped_bet(bet, cap);
    }
    println!("After 10 bets of {bet} the balance is ${}", user.get_balance());
}