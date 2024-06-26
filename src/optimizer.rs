/// Optimizer
/// Optimizers help to facilitate a faster gradient descent optimization
/// by utilizing more contextual information to speed up the gradient descent process

/// Gradient descent optimization:
/// θ = θ − η ⋅ ∇θ T(θ).
/// Minimize target function T by updating parameters to opposite direction of
/// derivative of the target function with respect to the params (e.g w, b)
/// Hence w - η ⋅ dT/dw or b - η ⋅ dT/db)
pub mod adam;
pub mod amsgrad;
pub mod nadam;

use nanoserde::{DeBin, SerBin};
use ndarray::Array2;
use std::borrow::Cow;
use std::default::Default;
use strum_macros::{Display, EnumString};

use crate::optimizer::{adam::Adam, amsgrad::AmsGrad, nadam::NAdam};

#[derive(Copy, Clone, Debug, Default, Display, EnumString, DeBin, SerBin, PartialEq)]
pub enum Optim {
    #[default]
    Default,
    Adam,
    AMSGrad,
    NAdam,
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum ParamKey {
    WeightGradient(u8),
    BiasGradient(u8),
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum HistType {
    // Historical Type
    Mean,
    Variance,
    Vhat,
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct CompositeKey(ParamKey, HistType);

pub trait Optimizer {
    fn calculate<'a>(
        &mut self,
        _key: ParamKey,
        value: &'a Array2<f64>,
        _t: usize,
    ) -> Cow<'a, Array2<f64>> {
        Cow::Borrowed(value)
    }
}

impl From<Optim> for Box<dyn Optimizer> {
    fn from(optimzer_type: Optim) -> Self {
        match optimzer_type {
            Optim::Default => Box::new(OptimDefault),
            Optim::Adam => Box::new(Adam::new()),
            Optim::AMSGrad => Box::new(AmsGrad::new()),
            Optim::NAdam => Box::new(NAdam::new()),
        }
    }
}

impl std::fmt::Debug for Box<dyn Optimizer> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Some optimizer variant")
    }
}

#[derive(Debug)]
pub struct OptimDefault; // default optimizer is none

impl Optimizer for OptimDefault {}
