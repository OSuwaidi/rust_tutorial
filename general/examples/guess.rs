// بسم الله الرحمن الرحيم وبه نستعين

use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io::{stdin, Error};

fn main() -> Result<(), Error> {
    let num = thread_rng().gen_range(1..=100);
    println!("Enter a number between 1 and 100:");
    loop {
        let mut guess_input = String::new();
        stdin().read_line(&mut guess_input)?;
        let guess_num: u32 = match guess_input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                if guess_input.trim().to_lowercase() == "quit" {
                    break;
                } else {
                    println!("Enter a number please");
                    continue;
                }
            }
        };
        if guess_num > 100 || guess_num < 1 {
            println!("Please enter a number between 1 and 100 only");
        } else {
            match guess_num.cmp(&num) {
                Ordering::Greater => println!("Too large"),
                Ordering::Less => println!("Too small"),
                Ordering::Equal => {
                    println!("Bingo! It was {num}");
                    return Ok(());
                }
            }
        }
    }
     Ok(()) // implicit "return"
}
