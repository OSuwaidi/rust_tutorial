// بسم الله الرحمن الرحيم وبه نستعين

use std::collections::HashMap;

enum Value<'a> {
    Str(&'a str), // we are enforcing that the "Str" variant references do not outlive the data they point to, avoiding dangling references
    Int(i32),
}

fn main() {
    let mut hmap = HashMap::new();
    // Using the "enum" "Value", we can insert into our hashmap values of different datatypes, belonging to a single type "Value"
    hmap.insert("string", Value::Str("cat"));
    hmap.insert("integer", Value::Int(10));

    for (_k, v) in &hmap {
        // we had NOT to *de*reference "v" because the "Value" variant doesn't implement the "Copy" trait, so we can't copy it
        match v {
            Value::Str(value) => println!("Value is a string: {value}"),
            Value::Int(value) => println!("Value is an int: {value}"),
        }
    }

    // Check if "integer" key has value > 5, if so, return true, otherwise false, else return error on every other possible case:
    println!(
        "Get result: {:?}",
        check_get_value(hmap.get("integer")).unwrap()
    );
}

fn check_get_value(get_result: Option<&Value>) -> Result<bool, &'static str> {
    return match get_result {
        // must use "return" before "match" to terminate function upon success/fail
        Some(value) => {
            match value {
                Value::Str(_string) => Err("Value is not an integer!"), // this "Err()" returns type "&'static str"
                Value::Int(int) => {
                    if int > &5 {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                }
            }
        }
        None => Err("The key 'integer' doesn't exist!"),
    };
}
