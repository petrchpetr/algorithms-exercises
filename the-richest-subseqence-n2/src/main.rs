use std::io;
use std::{fs::File, io::Read};




fn read(path: &str) -> Result<Vec<i64>, io::Error> {
    let mut file = File::open(path).expect("Can't open file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read to line.");

    let mut vec_numbers = Vec::<i64>::new();
    for line in contents.lines().into_iter().filter(|line| !line.is_empty() ) {
        vec_numbers.push(line.parse::<i64>().unwrap());
    }
    Ok(vec_numbers)
}

fn max(a: i64, b: i64) -> i64 {
    if a > b {
        a
    } else {
        b
    }
}


fn main() {
    println!("Find a subsequence with the largest sum");
    // let mut k = 0;
    let numbers = read("numbers.txt").unwrap();
    let n = numbers.len();
    let mut m = 0;
    let mut k = 0;

    for i in 0..n {
        // when the previous sum is negative, we are free to start with zero
        k = max(k,0)+numbers[i];
        // after adding the next element, either it is a new maximum or not
        m = max(m, k);
    }
    println!("m is {}", m);
}
