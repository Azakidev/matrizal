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
            let matx = spmatrix(matx, row, col);
            return (matx,row,col);           
        } else if query == 2{
            let matx = rngmatrix(matx, row, col);
            return (matx,row,col);
        } else {
            println!("Not a valid answer");
        }
    }

}

//Generates specific matrix

pub fn spmatrix (mut matx:matrix::format::Compressed<i32>, m:usize, n:usize) -> matrix::format::Compressed<i32> {

    println!("Type each value individually, filling each row progressively from left to right");

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

        }
    } 


    return matx
}

//Generates a random matrix of a specific order

pub fn rngmatrix(mut matx:matrix::format::Compressed<i32>,m:usize,n:usize) -> matrix::format::Compressed<i32> {

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

pub fn printmatx(matx:&matrix::format::Compressed<i32>,m:usize,n:usize) {

    for a in 0..m {
        print!("( ");
        for i in 0..n {

            print!("{}  ", &matx.get((a,i)))
        }
        print!(")");
        println!();
    } 
    println!();
}

//Prints determinant

pub fn printdet(matx:&matrix::format::Compressed<i32>,m:usize,n:usize,det:i32) {

    for a in 0..m {
        print!("| ");
        for i in 0..n {

            print!("{}  ", &matx.get((a,i)))
        }
        print!("|");
        println!();
    } 
    println!();
    println!("The determinant is {}",det);
}

//Transpose a matrix

pub fn transmatx (matx:matrix::format::Compressed<i32>,m:usize,n:usize) -> (matrix::format::Compressed<i32>,usize,usize) {

    let mut matx_t = Compressed::zero((m,m));

    for a in 0..m {
        
        for i in 0..n {

                matx_t.set((a,i), matx.get((i,a)));
        }
    } 
    return (matx_t,m,n)

}

//Creates identity matrix
#[allow(dead_code)]
pub fn idmatx (s:usize) -> (matrix::format::Compressed<i32>,usize,usize) {

    let m = s;
    let n = s;
    let mut matx = Compressed::zero((m,m));

    for a in 0..m {
        
        for i in 0..n {
            if a == i {
                matx.set((a,i), 1);
            } else {
                matx.set((a,i), 0)
            }
        }
    } 
    return (matx,m,n)

}

//Escalate specific matrix, just in case

pub fn esc_mult_matx (mut matx:matrix::format::Compressed<i32>,m:usize,n:usize,k:i32) -> (matrix::format::Compressed<i32>,usize,usize) {

    for a in 0..m {
        
        for i in 0..n {

            matx.set((a,i), matx.get((a,i)) * k);
        }
    } 
    return (matx, m, n);
}

//Matrix multiplication
#[allow(dead_code)]
pub fn mtmatx (a:matrix::format::Compressed<i32>,m:usize,n:usize,b:matrix::format::Compressed<i32>,j:usize,k:usize) -> (matrix::format::Compressed<i32>,usize,usize) {
    //Make matrixes
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

            return (c,m,k);
        } else {println!("Matrixes aren't square! (Current limitation)"); return (c,m,k)}
    } else {
        println!("These matrixes cannot be multiplied (A's colums do not match B's colums)"); return (c,m,k)
    }
}

//Determinants

pub fn detcalc (matx:&matrix::format::Compressed<i32>,s:usize) -> i32 {

    match s {
        2 => return matx.get((0,0))*matx.get((1,1))- matx.get((0,1))*matx.get((1,0)),
        
        3 => return matx.get((0,0))*matx.get((1,1))*matx.get((2,2))
                    +matx.get((0,1))*matx.get((1,2))*matx.get((2,0))
                    +matx.get((0,2))*matx.get((1,0))*matx.get((2,1))
                    -matx.get((0,2))*matx.get((1,1))*matx.get((2,0))
                    -matx.get((0,0))*matx.get((1,2))*matx.get((2,1))
                    -matx.get((0,1))*matx.get((1,0))*matx.get((2,2))
                    ,
        _ => {println!("Unsupported size!"); return 0;},
    };
}