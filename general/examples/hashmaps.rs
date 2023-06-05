// بسم الله الرحمن الرحيم وبه نستعين

use std::collections::HashMap;

fn main() {
    let mut dict = HashMap::new(); // need to specify datatypes for key, value pairs upon initializing the hashmap
    let k = "apple";
    let v = 10_i16;
    dict.insert(k, v);
    dict.insert("banana", 15);

    // Slower: accessing elements via references
    for kv in &dict {
        println!("key is {} and val is {}", kv.0, kv.1)
    }

    // Faster: accessing elements directly by value via dereferencing
    for (&k, &v) in &dict {
        println!("Key: {k}, Value: {v}");
    }

    if dict.contains_key("banana") {
        println!("We have a banana!");
    }
    let banana_price = dict.get("banana").expect("There are no bananas"); // ".get" returns an "Option" enum type, with "Some" or "None" variants
}
