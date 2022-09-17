use std::io;

use rand::{self, thread_rng, Rng};
use matrix::{*, prelude::Compressed};

//Makes a new matrix

pub fn newmatx() -> (matrix::format::Compressed<i32>,usize,usize) {

    //Query the size of the matrix
    let (row,col) = sizeq();
    let matx = Compressed::zero((row,col));
    //Random or specific
     
    loop {
        
        let mut query = String::new();

        println!("Do you want a specific matrix or a random one? (1/2)");

        io::stdin()
            .read_line(&mut query)
            .expect("Failed to read line");
            
            
        let query = match query.trim().parse() {
            Ok(query) => query,
            Err(_) => {0},};

        if query == 1 {        
            let matx = spmatrix(row, col, matx);
            return (matx,row,col);           
        } else if query == 2{
            let matx = rngmatrix(row, col, matx);
            return (matx,row,col);
        } else {
            println!("Not a valid answer");
        }
    }

}

//Generates specific matrix

pub fn spmatrix (row:usize, col:usize,mut matx:matrix::format::Compressed<i32>) -> matrix::format::Compressed<i32> {

    println!("Type each value individually, filling each row progressively from left to right");

    let m = row;
    let n = col;

    for a in 0..m {
        
        for i in 0..n {
            
            let mut num = String::new();

            io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

            let num = match num.trim().parse() {
                Ok(num) => num,
                Err(_) => {println!("Not a valid number, filling with 0"); 0},
            };
            
            matx.set((a,i), num);
            println!("Next");
        }
    } 


    return matx
}

//Generates a random matrix of a specific order

pub fn rngmatrix(row:usize,col:usize,mut matx:matrix::format::Compressed<i32>) -> matrix::format::Compressed<i32> {

    let m = row;
    let n = col;

    for a in 0..m {
        
        for i in 0..n {

            let mut rng = thread_rng();
            let n:i32 = rng.gen_range(-20..20);
            
            matx.set((a,i), n);
        }
    } 
    return matx
}

//Requests a size

pub fn sizeq() -> (usize, usize){

    //  Query

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

    //  Conversion

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

//Opposite of a matrix

pub fn oppmatrix(mut matx:matrix::format::Compressed<i32>,row:usize,col:usize) -> matrix::format::Compressed<i32> {
    let m = row;
    let n = col;

    for a in 0..m {
        
        for i in 0..n {

            matx.set((a,i), matx.get((a,i)) * -1);
        }
        
    } 
    return matx
}