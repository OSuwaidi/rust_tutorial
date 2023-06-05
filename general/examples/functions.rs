// بسم الله الرحمن الرحيم وبه نستعين

use rand::random;
use std::time::Instant;

fn measure_time(fun: fn(usize) -> Vec<i8>, n: usize) -> (u128, Vec<i8>) {
    let start = Instant::now();
    let vec = fun(n);
    (start.elapsed().as_nanos(), vec)
}

fn gen_rand1(n: usize) -> Vec<i8> {
    let mut vec: Vec<i8> = Vec::new();
    for _ in 0..n {
        vec.push(random::<i8>())
    }
    vec
}

fn gen_rand2(n: usize) -> Vec<i8> {
    let mut vec = vec![0; n];
    for e in &mut vec {
        *e = random::<i8>()
    }
    vec
}

fn gen_rand3(n: usize) -> Vec<i8> {
    (0..n).map(|_| random::<i8>()).collect()
    /*
    The "(0..n)" expression creates an iterator from "0" to "n" (exclusive).
    The "map()" method called on this iterator transforms each value in the range using the provided *closure* (lambda function).
    The closure "|_| rng.gen::<i8>()" generates a random "i8" value for each value in the range, while
    discarding the original value (which is why the "_" wildcard is used).
    So, the closure "|_| rng.gen::<i8>()" is essentially saying, "For each value in the range, discard the actual value,
    and generate a random "i8" number instead."
    The "collect()" method builds a "Vec<i8>" from the iterator containing these random numbers.
    */
}

fn main() {
    let (time1, rands1) = measure_time(gen_rand1, 10);
    println!("Took {time1} nanoseconds for {:?}", rands1);

    let (time2, rands2) = measure_time(gen_rand2, 10);
    println!("Took {time2} nanoseconds for {:?}", rands2);

    let (time3, rands3) = measure_time(gen_rand3, 10);
    println!("Took {time3} nanoseconds for {:?}", rands3);
}
