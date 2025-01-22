use std::io::{self, Write};

fn main() {
    println!("Welcome to the Number Guessing Game!\n");
    println!("I'm thinking of a number between 1 and 100.\n");

    let level = loop {
        println!("Please select the difficulty level:");
        println!("1. Easy (10 chances)");
        println!("2. Medium (5 chances)");
        println!("3. Hard (3 chances)");

        print!("\nEnter your choice: ");
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

    println!("\nGreat! You have selected the {} difficulty level. So you have {} chances.", difficulty, chances);
    println!("Let's start the game!");

// Enter your guess: 50
// Incorrect! The number is less than 50.

// Enter your guess: 25
// Incorrect! The number is greater than 25.

// Enter your guess: 35
// Incorrect! The number is less than 35.

// Enter your guess: 30
// Congratulations! You guessed the correct number in 4 attempts.
}
