// بسم الله الرحمن الرحيم وبه نستعين

use rand::{thread_rng, Rng};

fn main() {
    let password = gen_pass(15);
    println!("Password is: {:?}", password);
}

fn gen_pass(n: usize) -> String {
    let mut pass = String::with_capacity(n);
    for _ in 0..n {
        let c = thread_rng().gen_range(33..123) as u8;
        pass.push(c as char);
    }
    pass
}
