use rand::Rng;
use std::io;
fn main() {
    println!("Guess the number!");
    println!("Please in put your guess: ");
    let secretNumber = rand::thread_rng().gen_range(1, 101);
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
    println!("You guessed: {}", guess);
}
