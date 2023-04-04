// بسم الله الرحمن الرحيم وبه نستعين

fn main() {
    let t = [0; 50]; // repeats the value "0" 50 times
    println!("{:?}", t);

    let arr = [1, 2, 3, 4, 5, 6, 7, 8]; // arrays are fixed-size and can only contain one datatype!
    println!("First element: {}", arr[0]);
    println!("Length of array: {}", arr.len());
    let mut i = 0;
    loop {
        if arr[i] % 2 == 0 {
            i += 1;
            continue;
        }
        if arr[i] == 7 {
            break;
        }
        println!("{}", arr[i]);
        i += 1;
    }

    let mut i = 0;
    while i < arr.len() {
        println!("val1: {}", arr[i]);
        i += 1;
    }

    for val in arr {
        println!("val2: {}", val);
    }

    for i in 0..arr.len() {
        println!("val3: {}", arr[i]);
    }
}
