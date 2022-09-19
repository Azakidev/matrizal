mod operations;
mod modules;

use operations::*;

fn main() {

    println!("
Welcome to matrizal, a simple matrix calculator, now in your terminal!

Please choose one of the following operations to perform

    1 - Sum 2 matrixes.
    2 - Subtract 2 matrixes.
        
    3 - Multiply a matrix by an escalar.
    4 - Multiply 2 matrixes together. (Square matrixes only)
    5 - Square a matrix (square only)

    6 - Calculate the determinant of a matrix. (Order 2 or 3)
        
    7 - Transpose a matrix.
    8 - Generate a random matrix.

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
        1 => {summatrix()},
        2 => {submatrix()},
        3 => {escmatrix()},
        4 => {multmatrix()},
        5 => {sqmatrix()},
        6 => {determatrix()},
        7 => {transmatrix()},
        8 => {randomatrix()},
        _ => {println!("Not a valid choice")},
    };
}