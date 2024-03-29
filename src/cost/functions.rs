pub mod quadratic;
pub mod cross_entropy;

use std::convert::From;
use ndarray::{Array2, ArrayView2};

use crate::cost::{Cost, functions::
                  {quadratic::Quadratic, cross_entropy::CrossEntropy}}; // Update (2)
use crate::activation::functions::Act;

pub trait FunctionCost {
    fn compute(y: &Array2<f64>, a: &Array2<f64>) -> f64;
    fn derivative(y: &Array2<f64>, a: &ArrayView2<f64>) -> Array2<f64>; // produces dc_da
    fn combine_derivative(dc_da: Array2<f64>, da_dz: Array2<f64>, _act: Act) -> Array2<f64> {
        dc_da * da_dz
    }
}

#[derive(Copy, Clone)]
pub enum Loss {
    Quadratic,
    CrossEntropy,
}

impl From<Loss> for Box<dyn Cost> {
    fn from(loss_type: Loss) -> Self {
        // Add new activation type here - Start (3)
        match loss_type {
            Loss::Quadratic => Box::new(Quadratic),
            Loss::CrossEntropy => Box::new(CrossEntropy),
        }
    }
}
