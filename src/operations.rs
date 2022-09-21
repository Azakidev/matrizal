use std::io;
use matrix::prelude::*;
use crate::modules::*;

//Sum of matrixes

pub fn summatrix ()  {

    println!("Matrix A");
    let (a,m,n) = newmatx();
    println!();println!("Matrix B");
    let (b,j,k) = newmatx();
    let c = addmatx(a, m, n, b, j, k);

    printmatx(&c, m, n)
}

//Substraction of matrixes

pub fn submatrix () {

    println!("Matrix A");
    let (a,m,n) = newmatx();
    println!();println!("Matrix B");
    let (b,j,k) = newmatx();

    let b = oppmatrix(b, j, k);
    let c = addmatx(a, m, n, b, j, k);
    printmatx(&c, m, n)
}

//Matrix multiplied by escalar

pub fn escmatrix () {
    
    let (matx,m,n) = newmatx();                             //Declaration


    println!("Choose the factor you wanna multiply the matrix by");                             //Query

    let mut k = String::new();

        io::stdin()
        .read_line(&mut k)
        .expect("Failed to read line");

    let k = match k.trim().parse() {
        Ok(k) => k,
        Err(_) => {println!("Incorrect input, using 1");1},
    };
    

    let (matx, m, n) = esc_mult_matx(matx, m, n, k);          //Operation

    printmatx(&matx, m, n);
}

//Matrix multiplication

pub fn multmatrix () {
   
    //Make matrixes
    println!("Matrix A");
    let (a,m,n) = newmatx();
    println!();println!("Matrix B");
    let (b,j,k) = newmatx();

    //Check if it's possible and make result matrix
    if n == j {
    
    //Multiplication

        let (c,m,k) = mtmatx(&a, m, n, &b, j, k);

            println!();println!("Matrixes to multiply (A*B)"); println!();
            printmatx(&a, m, n); println!(); printmatx(&b, j, k);
            println!(); println!("C = A*B ="); println!();
            printmatx(&c, m, k);
        } else {
        println!("These matrixes cannot be multiplied (A's colums do not match B's colums)")
    }
}

//Square a matrix

pub fn sqmatrix () {
    let (a,m,n) = newmatx();
    let (c,m,n) = mtmatx(&a, m, n, &a, m, n);

    printmatx(&c, m, n);
}

//Determinant calculation

pub fn determatrix () {
    
    let (a,s,m) = newmatx();

    let d = detcalc(&a, s);
    printdet(&a, s, m, d)
}

//Transpose a matrix

pub fn transmatrix() {

    let (matx,m,n) = newmatx();
    let (matx,m,n) = transmatx(matx, m, n);
    printmatx(&matx, m, n);
}

//Random matrix

pub fn randomatrix() {

    let (m,n) = sizeq();
    let matx = Compressed::zero((m,n));
    let matx = rngmatrix(matx, m, n);
    printmatx(&matx, m, n);
}

//Opposite of a matrix

pub fn oppmatrix(matx:matrix::format::Compressed<i32>,m:usize,n:usize) -> matrix::format::Compressed<i32> {

    let (matx,_m,_n) = esc_mult_matx(matx, m, n, -1);
        
    return matx
}