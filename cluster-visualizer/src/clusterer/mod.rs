pub mod none;
pub mod k_means;
pub mod gaussian_mixture;
pub mod agglomerative_single_linkage;

pub use none::NoneClusterer;
pub use k_means::KMeansClusterer;
pub use gaussian_mixture::GaussianMixtureClusterer;
pub use agglomerative_single_linkage::AgglomerativeSingleLinkageClusterer;

use cluster::math::Vector;

use rand::prelude::*;
use rand_distr::Normal;

use crate::render::Render;

const SAMPLE_CLUSTER_COUNT : usize = 10;
const SAMPLE_CLUSTER_SIZE  : usize = 40;

const SAMPLE_CLUSTER_VARIANCE_MIN : f64 = 0.01;
const SAMPLE_CLUSTER_VARIANCE_MAX : f64 = 0.03;

const SAMPLE_COUNT : usize = SAMPLE_CLUSTER_COUNT * SAMPLE_CLUSTER_SIZE;
const CLUSTER_COUNT : usize = 10;

pub trait Clusterer {
    fn new_random() -> Box<Self> where Self: Sized {
        let mut rng = thread_rng();
        let mut sample_values = Vec::new();
        for _ in 0..SAMPLE_CLUSTER_COUNT {
            let mean_x = rng.gen_range(0.1..0.9);
            let mean_y = rng.gen_range(0.1..0.9);

            let var_x = rng.gen_range(SAMPLE_CLUSTER_VARIANCE_MIN..=SAMPLE_CLUSTER_VARIANCE_MAX);
            let var_y = rng.gen_range(SAMPLE_CLUSTER_VARIANCE_MIN..=SAMPLE_CLUSTER_VARIANCE_MAX);

            let dist_x = Normal::<f64>::new(mean_x, var_x).unwrap();
            let dist_y = Normal::<f64>::new(mean_y, var_y).unwrap();

            for _ in 0..SAMPLE_CLUSTER_SIZE {
                let x = dist_x.sample(&mut rng).max(0.05).min(0.95);
                let y = dist_y.sample(&mut rng).max(0.05).min(0.95);
                sample_values.push(Vector::from_array([x, y]));
            }
        }
        Self::from_sample_values(sample_values)
    }

    fn from_sample_values(sample_values : Vec<Vector<2>>) -> Box<Self> where Self: Sized;
    fn into_sample_values(self : Box<Self>) -> Vec<Vector<2>>;

    fn update(&mut self);
    fn render(&self, render : Render<'_>);
}

