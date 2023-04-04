// بسم الله الرحمن الرحيم وبه نستعين

use std::collections::HashMap;

fn main() {
    let mut dict = HashMap::new();
    let k = "apple";
    let v = 10_i16;
    dict.insert(k, v);
    dict.insert("banana", 15);
    for kv in &dict {
        println!("key is {} and val is {}", kv.0, kv.1)
    }

    if dict.contains_key("banana") {
        println!("We have a banana!");
    }
    let banana_price = dict.get("banana").expect("There are no bananas");
}
