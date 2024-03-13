mod complex;
mod errors;
mod matrix;
mod traits;
mod vector;
mod linear_regression;

extern crate csv;
use std::error::Error;

use linear_regression::data_loader;



fn main() -> Result<(), Box<dyn Error>> {
    data_loader("data.csv");
    Ok(())
}
