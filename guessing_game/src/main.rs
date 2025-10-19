// Guessing game: a game were the user has to guess a number based on hints from the computer.
use rand::Rng;
use std::io;

fn get_integer_input() -> i16 {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to parse line.");

    // Convert input string into integer.
    input.trim().parse::<i16>().expect("Please enter an integer.")
}

fn main() {

    // Generate a random number between 1 and 10 (inclusive).
    let mut rng = rand::rng();
    let number: i16 = rng.random_range(1..=10);

    loop {
        println!("Guess the number: ");
        let users_guess = get_integer_input();

        if users_guess > number {
            println!("Your guess is too high!");
        }

        if users_guess < number {
            println!("Your guess is too low!");
        }
        
        if users_guess == number {
            break;
        }
    }
    println!("My random number is {}", number);
}
