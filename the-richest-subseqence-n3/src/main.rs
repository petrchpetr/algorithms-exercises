use std::io;
// use std::io::BufReader;
// use std::io::Error;
// use std::io::ErrorKind;
//use std::fs::File;
use std::{fs::File, io::Read};


// function takes path in the form of string slice and returns enum
// which contains vector of integers on success or IO error type, see `std::io::Error`
// fn read(path: &str) -> Result<Vec<i64>, io::Error> {
//     let file = File::open(path)?; // open file by given path
//     // wrap file into generic buffered reader, it will use 4 KB buffer internally
//     // to reduce number of syscalls, thus improving performance
//     let br = BufReader::new(file);
//     // create an empty vector, type of the stored elements will be inferred
//     let mut v = Vec::new();
//     // br.lines() creates an iterator over lines in the reader
//     // see: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines
//     for line in br.lines() {
//         // IO operations generally can return error, we check if got
//         // an error,in which case we return this error as the function result
//         let line = line?;
//         let n = line   
//             .trim() // trim "whitespaces"
//             .parse() // call `str::parse::<i64>(&self)` method on the trimmed line, which parses integer
//             .map_err(|e| Error::new(ErrorKind::InvalidData, e))?; // parse() can return error (e.g. for string "abc"), here if we got it, we convert it to `std::io::Error` type and return it as function result
//         v.push(n); // push acquired integer to the vector
//     }
//     Ok(v) // everything is Ok, return vector
// }

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


fn main() {
    println!("Hello, world!");
    // let mut k = 0;
    let numbers = read("numbers.txt").unwrap();
    let n = numbers.len();
    let mut m = 0;
    for i in 0..n {
        let mut s=0;
        for j in i..n {
            s += numbers[j]
            // if numbers[j] > numbers[j+1] {
            //     let temp = numbers[j];
            //     numbers[j] = numbers[j+1];
            //     numbers[j+1] = temp;
            // }
        }
        if s > m {
            m = s;
        }
    }
    println!("m is {}", m);
    // for number in numbers {
    //     println!("number is {}", number);
    //     println!("")
    // }
}
