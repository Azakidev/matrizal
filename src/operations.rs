use std::io::{stdin, self};

use matrix::prelude::*;

use crate::modules::*;

pub fn matxsum ()  {
    
    let (a,m,n) = newmatx();
    let (b,j,k) = newmatx();
    let mut c = Compressed::zero((m,n));

    if m == j && n == k {

        for r in 0..m {
            
            for x in 0..n {

                c.set((r,x), a.get((r,x)) + b.get((r,x)));

            }
        } 

    } else {
        println!("The matrixes don't match! Returning 0mxn");
    }

    printmatx(&c, m, n)
}

pub fn matxsub () {

    let (a,m,n) = newmatx();
    let (b,j,k) = newmatx();
    let b = oppmatrix(b, j, k);
    let mut c = Compressed::zero((m,n));

    if m == j && n == k {

        for r in 0..m {
            
            for x in 0..n {

                c.set((r,x), a.get((r,x)) + b.get((r,x)));

            }
        } 

    } else {
        println!("The matrixes don't match! Returning 0mxn");
    }

    printmatx(&c, m, n)
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

//Matrix multiplied by escalar

pub fn escmatrix () {
    let (mut matx,m,n) = newmatx();                             //Declaration


    println!("Choose the factor you wanna multiply the matrix by");                             //Query

    let mut k = String::new();

        io::stdin()
        .read_line(&mut k)
        .expect("Failed to read line");

    let k = match k.trim().parse() {
        Ok(k) => k,
        Err(_) => {println!("Incorrect input, using 1");1},
    };
    

    let (matx, m, n) = esc_mult_matrix(matx, m, n, k);          //Operation

    printmatx(&matx, m, n);
}