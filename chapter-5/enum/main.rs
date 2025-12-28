#[derive(Clone, Copy)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coins: &[Coin]) -> i8 {
    let mut total: i8 = 0;
    for coin in coins {
        match coin {
            Coin::Penny => total += 1,
            Coin::Nickel => total += 5,
            Coin::Dime => total += 10,
            Coin::Quarter => total += 25,
        }
    }
    println!("total: {}", total);
    total
}

fn main() {
    let coins = [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter];
    println!("The total change is {}", value_in_cents(&coins))
}