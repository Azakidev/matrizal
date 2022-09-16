use std::io;

use rand::{self, thread_rng, Rng};
use matrix::prelude::*;

//Generates a random matrix of a specific order

pub fn rngmatrix(row:usize,col:usize) -> matrix::format::Compressed<i32> {

    let m = row;
    let n = col;
    let mut matx = Compressed::zero((m,n));

    for a in 0..m {
        
        for i in 0..n {

            let mut rng = thread_rng();
            let n:i32 = rng.gen_range(-20..20);
            
            matx.set((a,i), n);
        }
    } 
    return matx
}

//Prints matrix

pub fn printmatrix(matx:&matrix::format::Compressed<i32>,row:usize,col:usize) {
    let m = row;
    let n = col;

    for a in 0..m {
        
        for i in 0..n {

            print!("{} ", &matx.get((a,i)))
        }
        println!();
    } 
    println!();
}

//Requests a size

pub fn sizeq() -> (usize, usize){

    let mut row = String::new();
    let mut col = String::new();

    println!("Please choose the height of the matrix");

        io::stdin()
        .read_line(&mut row)
        .expect("Failed to read line");
    
    println!("Please choose the width of the matrix");
        
        io::stdin()
        .read_line(&mut col)
        .expect("Failed to read line");


    let row = match row.trim().parse() {
        Ok(n) => n,
        Err(_) => 2,
    };

    let col = match col.trim().parse() {
        Ok(n) => n,
        Err(_) => 2,
    };

    return (row,col)
}