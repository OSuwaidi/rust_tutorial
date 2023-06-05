// بسم الله الرحمن الرحيم وبه نستعين

fn main() {
    let tup = (10, "flower", 9.8); // when creating tuples, the different datatypes must be explicitly specified
    println!("Name: {}", tup.1);

    let (v1, v2, v3) = tup;
    println!("v1: {v1}, v2: {v2}, v3: {v3}");

    let mut text = String::new(); // new instance of a *growable* string type (stored on the heap)
    text.push('A'); // "push" can append for strings too
    text.push_str(" word");
    for word in text.split_whitespace() {
        println!("{word}");
    }
    let text2 = text.replace('A', "New");
    println!("{text2}");

    let text3 = String::from("a b c c c d e e f");
    let mut vec: Vec<char> = text3.chars().collect();
    println!("string_vec: {:?}", vec);
    // "chars()" converts a string into an iterator on its characters
    // "collect()" converts any iterator into a "collection" (vector, array, String)
    vec.sort();
    vec.dedup(); // removes any duplicates in the string
    for &e in &vec {
        println!("{e}")
    }
    let text4 = "some string";
    let text5 = text4.to_string();
    let text6 = &text5[0..4]; // not exclusive
    println!("String: '{text6}' has length: {}", text6.len());

    let text7 = String::from("Hello");
    let concat = text7 + " world"; // in rust, string concatenation must be of types: "String" + "%str"
    println!("Concatenated string: {concat}");
}
