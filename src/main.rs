use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn printdebug(message: String) {
    let debug: bool = false;
    if debug == true {
        println!("[DEBUG] {}", message)
    }
}

fn main() {
    println!("Knijn Guessing Game!");
    let secret_number: i128 = rand::thread_rng().gen_range(1..101); 
    printdebug("Debug mode is enabled".to_string());
    printdebug(format!("Secret Number: {}", secret_number).to_string());
    loop {

        println!("Please input your guess:");
        let mut guess: String = String::new();
        // read the io for the input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: i128 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        printdebug(format!("Guess: {}", guess));
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal =>  {
                println!("{}","You win! 🎉".green().bold());
                break;
            },
        }
    }
}
