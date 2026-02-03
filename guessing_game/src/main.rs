use rand::Rng;
use std::io;

fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1..=10);
  println!("secret number: {secret_number}");

  loop {
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");
    println!("You guessed: {guess}");

    let guess_number: u32 = guess.trim().parse().unwrap();
    if guess_number == secret_number {
      break;
    } else {
      println!("Try Again!");
    }
  }

  println!("Correct!");
}
