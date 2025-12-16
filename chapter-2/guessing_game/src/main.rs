use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1..=100);
  println!("The secret number is: {secret_number}");
  let mut tries = 3;
  let fail_case = 0;
  loop {
    match fail_case.cmp(&tries) {
      Ordering::Less => {
        println!("Please input your guess. Tries remaining: {tries}");
        tries -= 1;
      },
      Ordering::Greater => {
        println!("tries: {tries}")
      },
      Ordering::Equal => {
        println!("You have run out of chances. Sorry, you lose!");
        break;
      }
    } 
    let mut guess = String::new();
    io::stdin()
     .read_line(&mut guess)
     .expect("failed to read line");
    
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(err) => continue,
    };
      
    println!("You guessed: {guess}");
    
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}