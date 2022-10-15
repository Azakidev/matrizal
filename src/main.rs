mod operations;
mod modules;
mod bench;
#[allow(unused_imports)]
use bench::*;
#[allow(unused_imports)]
use operations::*;

fn main() {

    println!("
Welcome to matrizal, a simple matrix calculator, now in your terminal!

Please choose one of the following operations to perform

    0 - Benchmark menu ;)

    1 - Sum 2 matrixes.
    2 - Subtract 2 matrixes.
    3 - Linearly combine 2 matrixes
        
    4 - Multiply a matrix by an escalar.
    5 - Multiply 2 matrixes together.
    6 - 'Square' a matrix.

    7 - Calculate the determinant of a matrix. (Order 2 or 3)
        
    8 - Transpose a matrix.
    9 - Generate a random matrix.

Please input what you want to do");

    let mut i = String::new();

    //Query
    
    std::io::stdin()
        .read_line(&mut i)
        .expect("Failed to read line");

    //  Conversion

    let i = match i.trim().parse() {
        Ok(n) => n,
        Err(_) => 0,
    };

    match i {
        0 => {benchlist()},
        1 => {summatrix()},
        2 => {submatrix()},
        3 => {lcmatrix()},
        4 => {escmatrix()},
        5 => {multmatrix()},
        6 => {sqmatrix()},
        7 => {determatrix()},
        8 => {transmatrix()},
        9 => {randomatrix()},
        _ => {println!("Not a valid choice")},
    };
}

pub fn benchlist() {

    println!(
        
"Welcome to the benchmark menu! Here you can benchmark your device by automatically running 1000 of one specific operation.
This is a fun experiment mainly meant for myself to test how much my code sucks

Please input what you want to do

        0 - Go back
        
        1 - A + B
        
        2 - A * B

        3 - A * k

        4 - RNG(A)
"); 

    let mut i = String::new();

    //Query

    std::io::stdin()
        .read_line(&mut i)
        .expect("Failed to read line");

    //  Conversion

    let i = match i.trim().parse() {
        Ok(n) => n,
        Err(_) => 0,
    };

    match i {
        0 => {main()},
        1 => {benchsum()},
        2 => {benchmt()},
        3 => {benchesc()},
        4 => {benchrng()},
        _ => {println!("Not a valid choice")},
    };
}