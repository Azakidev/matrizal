mod modules;
use modules::{rngmatrix, printmatrix};

#[allow(non_snake_case)]
fn main() {
    let row = 6;
    let col = 6;
    
    let A = rngmatrix(row, col);
    
    printmatrix(&A, row, col);
}
