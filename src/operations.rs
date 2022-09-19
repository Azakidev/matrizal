use std::io;
use matrix::prelude::*;
use crate::modules::*;

//Sum of matrixes

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

//Substraction of matrixes

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
    let (a,m,n) = newmatx();
    let (b,j,k) = newmatx();
    let mut c = Compressed::zero((m,k));

    //Check if it's possible and make result matrix
    if n == j {
        
        if m == k && j == k {
    
    //Multiplication

        for o in 0..m {
            for p in 0..k {
                let mut sum = 0;
                for z in 0..m {
                    sum = sum + a.get((o,z)) * b.get((z,p));
                }
                c.set((o,p),sum);
            }
        }

            println!("Matrixes to sum (A*B)"); println!();
            printmatx(&a, m, n); println!(); printmatx(&b, j, k);
            println!(); println!("C = A*B ="); println!();
            printmatx(&c, m, k);
        } else {println!("Matrixes aren't square! (Current limitation)")}
    } else {
        println!("These matrixes cannot be multiplied (A's colums do not match B's colums)")
    }
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