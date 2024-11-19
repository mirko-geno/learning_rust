use roulette::User;


fn main() {
    let mut user = User::new(1000.0);

    let (bet, cap) = (10.0, 4);
    for _i in 0..100 {
        user.timbear(bet, cap);
    }
    println!("After 10 bets of {bet} the balance is ${}", user.get_balance());
}