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
        let samples = self.pixels().map(|pixel| Vector::from_array(Pixel::into_array(*pixel).map(Convert::convert))).collect::<Vec<_>>();
        let sample_count = samples.len();
        let cluster_count = k.get();
        match model {
            ClusterModel::KMeans => {
                let (cluster_means, sample_labels, _) = KMeans::new(sample_count, cluster_count).run(&samples, init, &mut thread_rng());
                for (sample_index, pixel) in self.pixels_mut().enumerate() {
                    let label = sample_labels[sample_index];
                    *pixel = Pixel::from_array(Vector::into_array(cluster_means[label]).map(Convert::convert));
                }
            },
            ClusterModel::GaussianMixture => {
                let (_, cluster_means, _, _, _, _, posteriors) = GaussianMixture::new(sample_count, cluster_count).run(&samples, init, &mut thread_rng());
                for (sample_index, pixel) in self.pixels_mut().enumerate() {
                    let label = (0..cluster_count).map(|cluster_index| posteriors[cluster_index * sample_count + sample_index]).position_max_by(f64::total_cmp).unwrap();
                    *pixel = Pixel::from_array(Vector::into_array(cluster_means[label]).map(Convert::convert));
                }
            },
        }
    }
}
