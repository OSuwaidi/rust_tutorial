// بسم الله الرحمن الرحيم وبه نستعين

fn main() {
    // "closures" are like "lambda" functions in python (one-liners)
    let can_vote = |age: u16| age >= 18;
    println!("Can vote: {}", can_vote(24));

    let add = |x: i32, y: i32| x + y;
    println!("8 + 9 = {}", add(8, 9));

    let words = ["cat", "dog", "ship", "tree", "sun"];
    let words = words.map(|w| capitalize(w));
    println!("Words: {:?}", words);
}

fn capitalize(string: &str) -> String {
    let mut chars = string.chars();
    let first_char = chars.next().unwrap().to_uppercase();
    first_char.chain(chars).collect()
}
