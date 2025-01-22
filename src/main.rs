use std::io::{self, Write};
use rand::Rng;

fn main() {
    println!("Welcome to the Number Guessing Game!");
    println!("I'm thinking of a number between 1 and 100.\n");
    let number: u8 = rand::thread_rng().gen_range(1..=100);

    let level = loop {
        println!("Please select the difficulty level:");
        println!("1. Easy (10 chances)");
        println!("2. Medium (5 chances)");
        println!("3. Hard (3 chances)");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut level = String::new();
        io::stdin().read_line(&mut level).unwrap();

        match level.trim().parse::<u8>() {
            Ok(level) if (1..=3).contains(&level) => break level, 
            _ => {
                println!("Invalid input. Please enter a number between 1 and 3.");
            }
        }
    };

    let levels = [("Easy", 10), ("Medium", 5), ("Hard", 3)];
    let difficulty = levels[(level -1 ) as usize].0; 
    let chances = levels[(level -1 ) as usize].1; 

    println!("Great! You have selected the {} difficulty level. So you have {} chances.", difficulty, chances);
    print!("\nLet's start the game!");

    for i in 0..chances {
        print!("\nEnter your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();

        let guess = match guess.trim().parse::<u8>() {
            Ok(guess) if (1..=100).contains(&guess) => guess, 
            _ => {
                println!("Invalid input. Please enter a number between 1 and 100.");
                continue;
            }
        };

        if guess == number {
            println!("Congratulations! You guessed the correct number in {} attempts.", i + 1);
            break;
        } else if guess < number {
            println!("Incorrect! The number is greater than {}.", guess);
        } else {
            println!("Incorrect! The number is less than {}.", guess);
        }
    }
}
