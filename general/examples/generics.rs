// بسم الله الرحمن الرحيم وبه نستعين

use std::f32::consts::PI;
use std::ops::Add;

/*
Using "<T>" is how you allow a function to accept "generic" datatypes or arguments, represented by "T"
fn add_generics<T>(x: T, y: T) -> T {
    x + y // this will give an error because you can't use the "add" operations on two or more generic types since the "Add" trait is not implemented for them
}
*/

// Therefore, we need to implement the "Add" trait for the generic datatypes:
fn add_generics<T: Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

fn main() {
    println!("5 + 7 = {}", add_generics(5, 7));
    println!("pi + 11.11 = {}", add_generics(PI, 11.11));
}
