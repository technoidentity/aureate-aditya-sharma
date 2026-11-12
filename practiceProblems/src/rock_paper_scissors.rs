use rand::Rng;
use std::io;

#[derive(Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

fn main() {
    println!("Welcome to RPS game!");
    let mut score: u32 = 0;
    loop {
        println!("Please input your guess (Use from rock, paper or scissors): ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Huh! OS issue. Try running again.");
        println!("If you want to quit then put `quit`.");
        let guess: String = guess.trim().to_lowercase();
        if guess == "quit" {
            println!("Thanks for playing!");
            break;
        }
        let random_index = rand::thread_rng().gen_range(0, 3);
        let moves: [String; 3] = [
            "rock".to_string(),
            "paper".to_string(),
            "scissors".to_string(),
        ];
        let computer_move: String = moves[random_index].clone();

        let outcome: Outcome = if guess == computer_move {
            Outcome::Draw
        } else if (guess == moves[0] && computer_move == moves[2])
            || (guess == moves[1] && computer_move == moves[0])
            || (guess == moves[2] && computer_move == moves[1])
        {
            Outcome::Win
        } else {
            Outcome::Loss
        };

        match outcome {
            Outcome::Win => {
                score += 3;
                println!("You won!")
            }
            Outcome::Loss => {
                println!("You lost!")
            }
            Outcome::Draw => {
                score += 1;
                println!("Its a draw")
            }
        }
    }
    println!("You scored {score} this time.");
}
