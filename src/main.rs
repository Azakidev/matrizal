mod operations;
mod modules;
use matrix::{prelude::Compressed, Matrix};

#[allow(unused_imports)]
use operations::*;
use modules::*;

#[allow(non_snake_case)]
fn main() {

    let matx = Compressed::zero(3);
    let row = 3;
    let col = 3;
    //let det = 0;
    let matx = rngmatrix(row, col, matx);
    
    printmatx(&matx,row,col);

}