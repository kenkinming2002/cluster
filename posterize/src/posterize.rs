use crate::Convert;
use crate::Pixel;
use crate::Image;

use cluster::math::*;
use cluster::model::ClusterModel;
use cluster::model::init::ModelInit;
use cluster::model::k_means::*;
use cluster::model::gaussian_mixture::*;

use itertools::Itertools;
use rand::prelude::*;
use std::num::NonZero;

/// Posterize an image.
pub trait Posterize {
    fn posterize(&mut self, model : ClusterModel, k : NonZero<usize>, init : ModelInit);
}

/// Implementation of [Posterize] trait for images.
///
/// This is done by applying the k-mean-clustering algorithm with parameter k and replacing each
/// pixel with the mean value of assigned cluster.
///
/// The last trait bound is a work-around for the inability to specify trait bounds on generic
/// associated constants. See [issue #104400](https://github.com/rust-lang/rust/issues/104400).
impl<I, P, C> Posterize for I
where
    I: Image<Pixel = P>,
    P: Pixel<Component = C>,
    C: Convert<f64>, f64: Convert<C>,
    [P::Component; P::COMPONENT_COUNT] : ,
{
    fn posterize(&mut self, model : ClusterModel, k : NonZero<usize>, init : ModelInit) {
        let samples = self.pixels().map(|pixel| Vector::from_array(Pixel::into_array(*pixel).map(Convert::convert)));
        match model {
            ClusterModel::KMeans => {
                let result = k_mean(&mut thread_rng(), init, k, samples);
                for (sample_index, pixel) in self.pixels_mut().enumerate() {
                    let label = result.labels[sample_index];
                    *pixel = Pixel::from_array(Vector::into_array(result.means[label]).map(Convert::convert));
                }
            },
            ClusterModel::GaussianMixture => {
                let result = gaussian_mixture(&mut thread_rng(), init, k, samples);
                for (sample_index, pixel) in self.pixels_mut().enumerate() {
                    let label = (0..result.cluster_count).map(|cluster_index| result.posteriors[cluster_index * result.sample_count + sample_index]).position_max_by(f64::total_cmp).unwrap();
                    *pixel = Pixel::from_array(Vector::into_array(result.cluster_means[label]).map(Convert::convert));
                }
            },
        }
    }
}
