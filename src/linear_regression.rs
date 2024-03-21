use std::{error::Error, io::Write};

use crate::csv;
use ft_matrix::matrix::Matrix;
use ft_matrix::vector::Vector;
use std::fs::File;
use plotly::{Plot, Scatter};


pub fn trainer_and_plotter()->Result<(),Box<dyn Error>>{
    let datas = data_loader("data.csv")?;
    let xdata = datas.column_extract(1)?;
    let xdata2 = datas.column_extract(1)?;
    let ydata = datas.column_extract(2)?;
    let ydata2 = datas.column_extract(2)?;
    let points = Scatter::new(xdata.matrix.elements.clone(), ydata.matrix.elements.clone()).name("Points").mode(plotly::common::Mode::Markers);

    let mut plot = Plot::new();
    plot.add_trace(points);
   
    
    let betas = ft_linear_regression(xdata, ydata)?;
    let predicted =Vector::linear_combination(&[&xdata2, &Vector::<f64>::ones(xdata2.size())?],
    &[betas.el(1)?, betas.el(2)?])?;
    let predicted_points = Scatter::new(xdata2.matrix.elements.clone(), predicted.matrix.elements.clone()).name("Points").mode(plotly::common::Mode::Lines);
  
    plot.add_trace(predicted_points);
    let residuals = predicted.sub(&ydata2)?;
    let res_points = Scatter::new(xdata2.matrix.elements.clone(), residuals.matrix.elements.clone()).name("Points").mode(plotly::common::Mode::Markers);
    let mut plot2 = Plot::new();
    plot2.add_trace(res_points);
    
    println!("{}", betas);
    let pred_v_real = Scatter::new(ydata2.matrix.elements.clone(), predicted.matrix.elements.clone()).name("Points").mode(plotly::common::Mode::Markers);
    let mut plot3 = Plot::new();
    plot3.add_trace(pred_v_real);

    let mut file1 = File::create("scatter.html")?;
    let mut file2 = File::create("residuals.html")?;
    let mut file3 = File::create("predicted_v_real.html")?;
    let mut file4 = File::create("betas.csv")?;

    file1.write_all(plot.to_html().as_bytes())?;
    file2.write_all(plot2.to_html().as_bytes())?;
    file3.write_all(plot3.to_html().as_bytes())?;
    write!(file4,"beta0,beta1\n{},{}", betas.el(1)?,betas.el(2)?)?;


    Ok(())
}

pub fn data_loader(path: &str) -> Result<Matrix<f64>, Box<dyn Error>> {
    let mut loading_vec: Vec<f64> = Vec::new();
    let input_file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(input_file);
    for res in rdr.records() {
        for field in res?.into_iter() {
            let value: f64 = field.parse::<f64>()?;

            loading_vec.push(value);
        }
    }

    let shape = loading_vec.len();

    Ok(Matrix::new(loading_vec, 2, shape / 2)?.tr()?)
}

pub fn ft_linear_regression(
    x: Vector<f64>,
    y: Vector<f64>,
) -> Result<Vector<f64>, Box<dyn Error>> {
    let mut betas = Vector::new(vec![0., 0.])?;
    let exes = x.matrix().clone();
    let ones = Vector::<f64>::ones(x.size())?;
    let (exes, ncoefs) = exes.normalize_cols()?;
    let exes = exes.append_horizontal(&ones.matrix())?;
    let nobs = x.size();
    let (y, m_y, sd_y) = y.normalize_vec()?;
    let alpha = 0.01 / ((2 * nobs) as f64);
    let mut cost;

    for _i in 0..10000 {
        let residual = exes.mul_vec(&betas)?.sub(&y)?;
        let jacob = exes.tr()?.mul_vec(&residual)?;
        cost = residual.norm() / (nobs as f64);
        betas = betas.sub(&jacob.scl(alpha)?)?;
        if cost < 1E-05{
            break;
        }
    }
    betas.set(
        2,
        -betas.el(2)?
            - betas.el(1)? * sd_y / ncoefs.el(2, 1)? * ncoefs.el(1, 1)?
            + m_y,
    )?;
    betas.set(1, betas.el(1)? * sd_y / ncoefs.el(2, 1)?)?;
    Ok(betas)
}
