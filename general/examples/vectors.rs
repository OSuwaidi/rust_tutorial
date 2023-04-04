// بسم الله الرحمن الرحيم وبه نستعين

fn main() {
    // Using arrays is faster than using vectors because arrays are stored in the stack, whereas vectors are stored in the heap
    let vec1: Vec<i32> = Vec::new(); // must specify vector's datatype when initializing
    let mut vec2 = vec![1, 2, 3, 4]; // vectors are dynamic-size collection as compared to arrays which are fixed-size collection
    vec2.push(5);
    println!("Last element: {}", vec2[vec2.len() - 1]);

    let x = vec2.get(0); // "get()" returns an "Option" type (exists or not), hence; it can only be compared to other "Option" types
    println!("Option is: {:?}", x); // using ":?" in format allows you to print un-formatted types
    println!("Value is: {}", x.unwrap());

    for e in &mut vec2 {
        // must assign "e" a *mutable reference* to "vec2", otherwise, "vec2"'s ownership will be moved to "e"!
        *e *= 2; // the "*" in the beginning of the variable allows you to manipulate the reference's ("vec2") values inplace
    }
    for &e in &vec2 {
        println!("{}", e);
    }
    println!("Popped value: {:?}", vec2.pop()); // "pop()" also returns an "Option" type
    println!(
        "Actual popped value: {}",
        vec2.pop().expect("Empty vector!")
    );

    if vec2.contains(&4) {
        // or you can use a "match" case
        // the method takes reference (pointer) as argument for efficiency instead of unnecessary data copying
        println!("4 exists!");
    } else {
        println!("4 doesn't exist");
    }
}
