// بسم الله الرحمن الرحيم وبه نستعين

use std::time::Instant;

fn measure_time(n_loops: usize, fun: fn(u128) -> u128, n: u128) -> f64 {
    let mut times = vec![0; n_loops];
    for i in 0..n_loops {
        let start = Instant::now();
        fun(n);
        times[i] = start.elapsed().as_nanos()
    }
    let sum: u128 = times.iter().sum(); // summing using "loop4" would've been faster, but this is more concise
    (sum as f64) / (n_loops as f64)
}

fn loop1(size: u128) -> u128 {
    let vec: Vec<u128> = (0..=size).collect();
    let mut sum = 0;
    for v in vec {
        sum += v
    }
    sum
}

fn loop2(size: u128) -> u128 {
    let vec: Vec<u128> = (0..=size).collect();
    let mut sum = 0;
    for v in &vec {
        sum += v
    }
    sum
}

fn loop3(size: u128) -> u128 {
    let vec: Vec<u128> = (0..=size).collect();
    let mut sum = 0;
    for v in vec {
        sum += &v
    }
    sum
}

fn loop4(size: u128) -> u128 {
    let vec: Vec<u128> = (0..=size).collect();
    let mut sum = 0;
    for &v in &vec {
        sum += v
    }
    sum
}

fn loop5(size: u128) -> u128 {
    let vec: Vec<u128> = (0..=size).collect();
    let mut sum = 0;
    for &v in &vec {
        sum += &v
    }
    sum
}

/*
loop1(): This function iterates over the vector by value, meaning that each element is copied during iteration, which is less efficient than using references.

loop2(): This function iterates over the vector by reference, avoiding copying the elements. However, the sum is updated using a reference.

loop3(): This function iterates over the vector by value, but adds the value by reference in the loop. This is less efficient than iterating by reference.

loop4(): This function iterates over the vector by reference and *dereferences* the reference inside the loop. This is the most efficient way to iterate over the vector and update the sum.

loop5(): This function iterates over the vector by reference but updates the sum using a reference, which is less efficient than loop4().

*** loop4 > loop5 > loop1 > loop2 > loop3

Adding values directly is faster than adding references of values!
*/

fn main() {
    let time1 = measure_time(1000, loop1, 100000);
    println!("Took {:?} nanoseconds for loop1", time1);

    let time2 = measure_time(1000, loop2, 100000);
    println!("Took {:?} nanoseconds for loop2", time2);

    let time3 = measure_time(1000, loop3, 100000);
    println!("Took {:?} nanoseconds for loop3", time3);

    let time4 = measure_time(1000, loop4, 100000);
    println!("Took {:?} nanoseconds for loop4", time4);

    let time5 = measure_time(1000, loop5, 100000);
    println!("Took {:?} nanoseconds for loop5", time5);
}
