mod modules;
use modules::{rngmatrix, printmatrix, sizeq};

#[allow(non_snake_case)]
fn main() {
    let (row,col) = sizeq();
    
    let A = rngmatrix(row, col);
    
    printmatrix(&A, row, col);
}
