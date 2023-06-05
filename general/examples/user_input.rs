// بسم الله الرحمن الرحيم وبه نستعين

use std::io::{stdin, Error}; // the "use" keyword is similar to "import"

// "Result<T, E>" --> "T" is type of successful value, and "E" is error type
fn main() -> Result<(), Error> {
    // "?" operator can only be used inside functions that return the "Result" type
    let mut input = String::new();
    stdin().read_line(&mut input)?; // "?" operator is used for more concise error handling
    let int_input: i32 = input.trim().parse().expect("Enter an integer please");
    /*
    ".trim()" removes trailing and leading white spaces (needed for the invisible character at the end of a terminal execution)
    ".parse()" reads the string slice into another datatype
    ".expect()" extracts and returns the value as the explicitly defined datatype, else returns error
    */

    if int_input >= 10 {
        println!("That's a great number!")
    } else if int_input <= 0 {
        println!("That's a teensy number!")
    } else {
        println!("Meh...")
    }

    println!("The integer input is: {int_input}");
    return Ok(());
}
