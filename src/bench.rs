use std::time::Instant;

use matrix::{prelude::Compressed, Matrix};

use crate::modules::*;

pub fn benchmt() {
    let now = Instant::now();

    let b = rngmatrix(Compressed::zero((9,9)),9,9);

    for _i in 0..1000 {
        let a = rngmatrix(Compressed::zero((9,9)),9,9);
        let (_b,_o,_p) = mtmatx(&a,9,9,&b,9,9);
    }

    let time = now.elapsed();
    println!("It took {:.2?} to complete", time);
}

pub fn benchesc() {
    let now = Instant::now();

    for i in 0..1000 {
        let a = rngmatrix(Compressed::zero((9,9)),9,9);
        let _a = escmatx(a, 9, 9, i);
    }

    let time = now.elapsed();
    println!("It took {:.2?} to complete", time);
}

pub fn benchsum() {
    let now = Instant::now();

    for _i in 0..1000 {
        let b = rngmatrix(Compressed::zero((9,9)),9,9);
        let a = rngmatrix(Compressed::zero((9,9)),9,9);
        let _c = addmatx(a, 9, 9, b, 9, 9);
    }

    let time = now.elapsed();
    println!("It took {:.2?} to complete", time);
}

pub fn benchrng() {
    let now = Instant::now();

    for _i in 0..1000 {
        let _b = rngmatrix(Compressed::zero((9,9)),9,9);
    }

    let time = now.elapsed();
    println!("It took {:.2?} to complete", time);
}