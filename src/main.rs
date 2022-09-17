mod operations;
mod modules;

#[allow(unused_imports)]
use matrix::{prelude::Compressed, Matrix};
#[allow(unused_imports)]
use operations::*;
#[allow(unused_imports)]
use modules::*;

#[allow(non_snake_case)]
fn main() {

    let (matx,row,col) = newmatx();
    let (matx,row,col) = transmatx(matx, row, col);
    printmatx(&matx,row,col);

}