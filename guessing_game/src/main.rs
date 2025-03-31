use colored::Colorize;
use rand::{Rng, thread_rng};
use std::{cmp::Ordering, io};

fn main() {
    println!("{}","Guess the number!".red().on_blue());
    let target = thread_rng().gen_range(1..=100);

    loop {
        println!("{}", "Please input your guess.".red().on_blue());
        let mut guess = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(_) => (),
            Err(_) => {
                println!("Error: {}", "Failed to read line".red().on_blue());
                continue;
            }
        }

        let guess = match guess.trim().parse::<u32>() {
            Err(_) => {
                println!("{}", "Please type a number".red().on_blue());
                continue;
            }
            Ok(num) => num,
        };
        if !(1..=100).contains(&guess) {
            println!(
                "{}",
                "Please type a number between 1 and 100".red().on_blue()
            );
            continue;
        }
        println!("You guessed: {guess}");

        match guess.cmp(&target) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
        }
    }
}
