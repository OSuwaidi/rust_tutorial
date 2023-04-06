// بسم الله الرحمن الرحيم وبه نستعين

use std::io::{stdin, Error};

// "Result<T, E>" is an "enum" with 2 variants: "Ok(T)" is type of successful value, and "Err(E)" is error type
fn main() -> Result<(), Error> {
    let mut n = String::new(); // Variable "n" owns the data "String::new()" and its allocated memory
    stdin().read_line(&mut n)?; // Pass in a *mutable reference* to "n" for the method to ***borrow***
    let m = &n; // Assign "m" a *view/reference* to the variable "n", otherwise ownership will *move* from "n" to "m" and "n" will no longer be accessible!
    print!("{} {}", n, m);

    let text1 = String::from("hello");
    let text2 = &text1[0..2]; // "text2" borrows "text1"
    let concat_text = text1 + " world"; // "concat_text" now owns "text1", hence; we can't call "text1" afterwards
    println!("{}", concat_text);
    // println!("Error: {}", text2); // error because "text1" was moved out to "concat_text" but "text2" still points to "text1" which has moved out (dangling pointer)
    return Ok(()); // Return "Ok" variant of success type "()" --> unit value (no meaningful value to return on success)
}

/*
In rust, every bit of memory has exactly ONE owner only, and as soon as the owner gets deleted
or is out of scope, its memory is immediately freed
*/

/*
Additionally, when you pass a variable to a function/method/loop or assign it to another variable,
the default behavior is to *move* ownership of that variable into the function/method/loop or the new variable,
and as such, the old variable will no longer be accessible since its ownership of that piece of memory has *moved*
*/

/*
Thus, we had to pass a mutable ***reference*** (&) to "n" instead of "n" directly into the "read_line()" method,
because otherwise, the ownership of "n" would be ***moved*** to the "read_line()" method, and then we would
not be able to access the "n" variable after the method call
*/

/*
Note: primitive datatypes: {integers, floats, booleans, tuples, arrays, and characters} all implement the "Copy" trait;
meaning they can be passed as arguments directly without using "&" and their value data will be automatically copied
*/

/*
*** BIG QUESTION ***
Q.) When should I *move* ownership of a variable, "copy" a variable, or pass a *reference* to a variable?

Ans:
    When dealing with simple datatypes (e.g., small "structs" or "enums"), moving their ownership by passing them directly (pass by value)
    is more efficient, because transferring their ownership is cheap and doesn't involve any copying of the data, especially
    if the function/method needs to mutate the value, and access to the variable is unneeded afterwards.

    When dealing with primitive datatypes that implement the "Copy" trait, you should pass their variables by value, because
    copying their data is faster since they're trivially copyable, and there is no need to manage their ownership or
    borrowing by passing them as references using the "&".

    When dealing with large or complex datatypes, it's more efficient to use a *reference* "&"; which essentially is a
    pointer to the original value, because it avoids the cost of copying (cloning) the data.
*/

/*
Note: when using a reference "&", it's essentially a pointer to an address in memory, and when used in an x64 CPU architecture, it's size is 64 bits (8 bytes)!
 */

/*
Note: you CAN'T have a mutable reference and an immutable reference to a variable at the same time!
Because one function/method can change the mutable reference to the variable while the other function/method expects it to remain the same
 */
