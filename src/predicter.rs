extern crate ft_matrix;


extern crate csv;
pub mod linear_regression;

use std::error::Error;

use linear_regression::{trainer_and_plotter};

fn main() -> Result<(), Box<dyn Error>> {
    trainer_and_plotter()?;
    Ok(())
}