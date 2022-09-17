mod operations;
use modules::{newmatx, oppmatrix, printmatrix};
use operations::*;
mod modules;
//use modules::*;

#[allow(non_snake_case)]
fn main() {

    matxsum();
    
    let (matx,row,col) = newmatx();
    let matx = oppmatrix(matx, row, col);
    printmatrix(&matx, row, col);

}