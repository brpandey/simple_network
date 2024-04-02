use ndarray::{Array2, Axis};
use std::{env, fmt};

pub mod csv;
pub mod idx;

pub const ROOT_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub trait DataSet  {
    fn fetch(&mut self, token: &str);
    fn train_test_split(&mut self, split_ratio: f32) -> TrainTestSubsetData;
}

//                           x_train    y_train        # train  x_test     y_test     # test
pub type TrainTestTuple = (Array2<f64>, Array2<f64>, usize, Array2<f64>, Array2<f64>, usize);
pub struct TrainTestSubsetData(TrainTestTuple);

#[derive(Copy, Clone)]
pub struct SubsetRef<'a> {
    pub x: &'a Array2<f64>,
    pub y: &'a Array2<f64>,
    pub size: usize,
}

//                                   train         test
pub type TrainTestSubsetRef<'a> = (SubsetRef<'a>, SubsetRef<'a>);

impl TrainTestSubsetData {

    // scale is 0 min 1 max
    pub fn min_max_scale(&self, min: f64, max: f64) -> Self {
        let x_train = &self.0.0;
        let x_test = &self.0.3;

        // get the min values for each column
        let x_train_mins = x_train.map_axis(Axis(0), |v| *v.iter().min_by(|a,b| a.total_cmp(b)).unwrap());
        let x_test_mins = x_test.map_axis(Axis(0), |v| *v.iter().min_by(|a,b| a.total_cmp(b)).unwrap());

        let x_train_maxs = x_train.map_axis(Axis(0), |v| *v.iter().max_by(|a,b| a.total_cmp(b)).unwrap());
        let x_test_maxs = x_test.map_axis(Axis(0), |v| *v.iter().max_by(|a,b| a.total_cmp(b)).unwrap());

        let x_train_std = (x_train - &x_train_mins) / (&x_train_maxs - &x_train_mins);
        let x_test_std = (x_test - &x_test_mins) / (&x_test_maxs - &x_test_mins);

        let x_train_scaled = x_train_std*(max-min) + min;
        let x_test_scaled = x_test_std*(max-min) + min;

        TrainTestSubsetData((x_train_scaled, self.0.1.clone(), self.0.2, x_test_scaled, self.0.4.clone(), self.0.5))
    }


    pub fn get_ref<'a>(&'a self) -> TrainTestSubsetRef<'a> {
        (
            SubsetRef {
                x: &self.0.0,
                y: &self.0.1,
                size: self.0.2
            },
            SubsetRef {
                x: &self.0.3,
                y: &self.0.4,
                size: self.0.5,
            }
        )
    }
}

impl fmt::Display for TrainTestSubsetData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x_train shape {:?}, y_train shape  {:?}, x_test shape {:?}, y_test shape {:?}",
                 &self.0.0.shape(), &self.0.1.shape(), &self.0.3.shape(), &self.0.4.shape())
    }
}





