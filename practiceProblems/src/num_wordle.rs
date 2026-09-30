use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!(
        "{}",
        "Enter a number determining the number of digits you want to take for your guess: ".green()
    );
    let mut digit = String::new();
    io::stdin()
        .read_line(&mut digit)
        .expect(&"Error reading number of digits.".red().to_string());
    let digit: i32 = digit
        .trim()
        .parse()
        .expect(&"Expected a numerical value!".red().to_string());
    // TODO: If the digit count is more than 18 then loop back to ask again for a valid smaller number.
    let mut cnt: i64 = 1;
    let mut num: i64 = 0;
    for _ in 0..digit {
        let random_digit: i64 = rand::thread_rng().gen_range(0, 10);
        num += random_digit * cnt;
        cnt *= 10;
    }

    loop {
        println!("Enter your guess or type {} to quit: ", "quit".red());
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect(&"Error in reading the guess.".red().to_string());
        if guess.trim() == "quit" {
            println!("{}", "Thanks for playing!".blue());
            break;
        }
        let guess: i64 = guess
            .trim()
            .parse()
            .expect(&"Pass a valid number to play this game".red().to_string());
        match guess.cmp(&num) {
            Ordering::Less => {
                println!("{}", "Try guessing a higher number.".blue());
            }
            Ordering::Greater => {
                println!("{}", "Try guessing a smaller number.".blue());
            }
            Ordering::Equal => {
                println!("{}", "You guessed it correctly! You Win!".green());
                break;
            }
        }
    }
}
