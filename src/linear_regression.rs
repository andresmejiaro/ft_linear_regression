use std::error::Error;


use crate::matrix::Matrix;
use crate::vector::Vector;
use crate::csv;
use std::fs::File;

pub fn data_loader(path: &str)-> Result<Matrix<f64>,Box<dyn Error>>{
    let mut loading_vec: Vec<f64> = Vec::new();
    let input_file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(input_file);
        for res in rdr.records() {
           for field in res{
                let value: f64 = field.into();
                loading_vec.push(value);
           }
           println!("{:?}", res?);
        }

    Ok(Matrix::new(vec![1.], 1,1)?)

}


fn ft_linear_regression(x: Matrix<f64>, y: Vector<f64>){

}