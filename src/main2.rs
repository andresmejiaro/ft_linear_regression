extern crate ft_matrix;
use std::{env, process};
extern crate csv;
pub mod linear_regression;

use std::error::Error;

use linear_regression::{data_loader};
use ft_matrix::matrix::Matrix;
use ft_matrix::vector::Vector;


fn main() -> Result<(), Box<dyn Error>> {
    if env::args().count()!= 2 {
        println!("Wrong Number of arguments");
        process::exit(1);
    }
    let parsed_km: Vec<String> = env::args().collect();
    let parsed_km: f64 = parsed_km[1].parse()?;
    
    //println!("parsed = {}", parsed_km);

    let betas_s = data_loader("betas.csv");

    let betas:Matrix<f64> = match betas_s {
        Err(_e) => {println!("Something wrong with betas.csv. setting to 0");
                                    Matrix::zero(1, 2)?},
        Ok(beta) => {beta},       
    }; 
    println!("{}",betas);
    let inp = Vector::<f64>::new(vec![parsed_km, 1.])?;
    let res = betas.mul_vec(&inp)?;
    println!("The predicted value is {}", res.el(1)?);

    Ok(())
}
