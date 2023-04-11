// بسم الله الرحمن الرحيم وبه نستعين

use rand::{random, thread_rng, Rng};
use std::cmp::Ordering;

fn main() {
    let rand_bool: bool = random(); // must specify datatype to generate random var accordingly
    let rand_num = thread_rng().gen_range(1..=100); // generate a random number between 1 and 100
    println!("Rand bool: {}, Rand num: {}", rand_bool, rand_num);

    if rand_bool || (rand_num > 50) {
        println!("One of them is true?")
    } else {
        println!("Neither are true :(")
    }

    if rand_bool && (rand_num > 50) {
        println!("Both are true!")
    }

    let var = if 1 < rand_num && rand_num < 10 {
        1000 // rust has *implicit returns* and the end of functions and conditions
    } else if 10 < rand_num && rand_num < 20 {
        2000
    } else {
        0 // if you put a  ";" at the end of a returned value WITHOUT the "return" keyword, it will return unit value "()" instead of the actual value
    };
    println!("var is: {}", var);

    let x = match rand_num {
        // "match" must be *exhaustive*
        1..=30 => 3,
        31..=61 => 6,
        99 | 100 => 100, // a single "|" is the "pattern OR" operator
        _ => 0,          // "_" implies everything else
    };
    println!("x is : {}", x);

    let x = 18;
    let y = 10;
    match x.cmp(&y) {
        Ordering::Less => println!("Value is less"),
        Ordering::Greater => println!("Value is greater"),
        Ordering::Equal => println!("Values are equal!"),
    };
}
