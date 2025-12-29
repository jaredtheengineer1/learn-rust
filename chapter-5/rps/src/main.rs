use std::io;
use rand::Rng;
#[derive(Debug)]
enum Choices {
  Rock,
  Paper,
  Scissors
}

fn main() {
  let mut player_1_choice:String = String::new();
  let rand_choice_num = rand::thread_rng().gen_range(0..3);
  let mut player_2_choice = Choices::Rock;
  println!("Computer is making its choice...");//{}",rand_choice_num);
  match rand_choice_num {
    0 => player_2_choice = Choices::Rock,
    1 => player_2_choice = Choices::Paper,
    2 => player_2_choice = Choices::Scissors,
    _ => (),
  }

  println!("Player, please choose 'Rock', 'Paper', or 'Scissors'");
  io::stdin().read_line(&mut player_1_choice).expect("failed to read line");
  let player_1_choice: Choices = match player_1_choice.trim().to_lowercase().as_str() {
    "rock" => Choices::Rock,
    "paper" => Choices::Paper,
    "scissors" => Choices::Scissors,
    _ => {
      println!("Invalid choice! Please choose Rock, Paper, or Scissors.");
      return;
    }
  };
  // println!("Player 1 chose: {:?}, Player 2 chose: {:?}", player_1_choice, player_2_choice);
  match (player_1_choice, player_2_choice) {
    (Choices::Rock, Choices::Scissors) => println!("Player 1 wins!"),
    (Choices::Rock, Choices::Paper) => println!("Player 2 wins!"),
    (Choices::Paper, Choices::Rock) => println!("Player 1 wins!"),
    (Choices::Paper, Choices::Scissors) => println!("Player 2 wins!"),
    (Choices::Scissors, Choices::Paper) => println!("Player 1 wins!"),
    (Choices::Scissors, Choices::Rock) => println!("Player 2 wins!"),
    _ => println!("It's a tie!"),
  }
}