// بسم الله الرحمن الرحيم وبه نستعين

use std::time::Instant;

fn measure_time(n_loops: usize, fun: fn(u64) -> u64, n: u64) -> f64 {
    let mut times = vec![0; n_loops];
    for i in 0..n_loops {
        let start = Instant::now();
        fun(n);
        times[i] = start.elapsed().as_nanos()
    }
    let sum: u128 = times.iter().sum(); // summing using "loop1" would've been faster, but this is more concise
    (sum as f64) / (n_loops as f64)
}

fn loop1(size: u64) -> u64 {
    let vec: Vec<u64> = (0..=size).collect();
    let mut sum = 0;
    for v in vec {
        // ownership of elements in "vec" is moved into the loop
        sum += v // adds values directly
    }
    sum
}

fn loop2(size: u64) -> u64 {
    let vec: Vec<u64> = (0..=size).collect();
    let mut sum = 0;
    for v in vec {
        // ownership of elements in "vec" is moved into the loop
        sum += &v // adds values indirectly via references (extra step)
    }
    sum
}

fn loop3(size: u64) -> u64 {
    let vec: Vec<u64> = (0..=size).collect();
    let mut sum = 0;
    for &v in &vec {
        // dereferences the referenced vector "vec"
        sum += v // adds values directly
    }
    sum
}

fn loop4(size: u64) -> u64 {
    let vec: Vec<u64> = (0..=size).collect();
    let mut sum = 0;
    for v in &vec {
        sum += v // adds values indirectly via references (extra step)
    }
    sum
}

fn loop5(size: u64) -> u64 {
    let vec: Vec<u64> = (0..=size).collect();
    let mut sum = 0;
    for &v in vec.iter() {
        // "iter()" returns a reference of "vec", then values are being dereferenced
        sum += v // adds values directly
    }
    sum
}

/*
Note: Adding values directly is faster than adding references of values!

*** Performance (the higher the better): ***
    loop1 > loop2 > loop3 = loop5 > loop4
*/

fn main() {
    let number_of_loops: usize = 500;
    let vector_size: u64 = 1000000;
    let time1 = measure_time(number_of_loops, loop1, vector_size);
    println!("Took {time1} nanoseconds for loop1");

    let time2 = measure_time(number_of_loops, loop2, vector_size);
    println!("Took {time2} nanoseconds for loop2");

    let time3 = measure_time(number_of_loops, loop3, vector_size);
    println!("Took {time3} nanoseconds for loop3");

    let time4 = measure_time(number_of_loops, loop4, vector_size);
    println!("Took {time4} nanoseconds for loop4");

    let time5 = measure_time(number_of_loops, loop5, vector_size);
    println!("Took {time5} nanoseconds for loop5");
}
