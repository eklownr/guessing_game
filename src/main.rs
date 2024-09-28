use std::io; 
use rand::Rng;
use std::cmp::Ordering;
use colored::*;


fn main() {
    println!("{}", "Guess the number!".color("yellow"));

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut count_guess: u32 = 0;

    loop {    
        println!("\nPlease input your guess.");
        count_guess += 1;
        println!("Guess number {}", count_guess);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{}{}", "You guessed:".color("yellow"), guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".color("blue")),
            Ordering::Greater => println!("{}", "Too big!".color("red")),
            Ordering::Equal => {
                println!("{}", "You win!".color("green"));
                print!("Only {} guesses!", count_guess);
                break;
            }
        }
    }
}
