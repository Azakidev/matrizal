use matrix::prelude::*;

use crate::modules::{newmatx, printmatrix, oppmatrix};

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

    printmatrix(&c, m, n)
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

    printmatrix(&c, m, n)
}