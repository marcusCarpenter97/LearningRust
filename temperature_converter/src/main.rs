// Converts celcious to fahenheit and vice versa.
use std::io;

fn fahenheit_to_celcius(temperature: i16) {
    let result = (temperature - 32) * 5 / 9;
    println!("{} F is {} celcius.", temperature, result);
}

fn celcius_to_fahenheit(temperature: i16) {
    let result = temperature * (9/5) + 32;
    println!("{} C is {} fahenheit.", temperature, result);
}

fn get_integer_input() -> i16 {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to parse line.");

    // Convert input string into integer.
    input.trim().parse::<i16>().expect("Please enter an integer.")
}

fn main() {
    
    loop {
        println!("Choose which temperature unit to convert from:
                    1. Celcius.
                    2. Fahenheit.");

        let option = get_integer_input();

        match option {

            1 => { println!("Enter the temperature as a number: ");
                   let temperature = get_integer_input();
                   celcius_to_fahenheit(temperature);
                   break }
        
            2 => { println!("Enter the temperature as a number: ");
                   let temperature = get_integer_input();
                   fahenheit_to_celcius(temperature);
                   break }

            _ => println!("Try again!")
        }
    }
}
