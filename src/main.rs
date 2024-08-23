use std::cmp::Ordering;
use std::io;
use std::process::Command;
use rand::Rng;

fn main() {
    println!("Welcome to guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess_counter: u16 = 0;

    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                num
            }
            Err(_) => {
                println!("Please Enter valid Number!");
                continue;
            }
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small"),
            Ordering::Greater => println!("To tall"),
            Ordering::Equal => {
                if cfg!(target_os = "windows") {
                    Command::new("cls")
                        .status().expect("Failed");
                } else {
                    Command::new("clear")
                        .status().expect("Failed");
                }
                println!("You guessed it! The Secret Number was {}", secret_number);
                println!("You took {} guesses to find the number", guess_counter +1);
                break;
            }
        }
        guess_counter += 1;
    }
}