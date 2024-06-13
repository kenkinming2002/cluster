use crate::Convert;
use crate::Pixel;
use crate::Image;

use cluster::math::*;
use cluster::model::init::ModelInit;
use cluster::model::k_means::*;
use cluster::model::gaussian_mixture::*;
use cluster::model::agglomerative::*;

use itertools::Itertools;
use rand::prelude::*;
use std::num::NonZero;

use clap::Subcommand;

#[derive(Debug, Clone, Copy, Subcommand)]
pub enum PosterizeMethod {
    KMeans { init : ModelInit, cluster_count : NonZero<usize>, },
    GaussianMixture { init : ModelInit, cluster_count : NonZero<usize>, },
    AgglomerativeSingleLinkage { cluster_count : NonZero<usize> },
}

/// Posterize an image.
pub trait Posterize {
    fn posterize(&mut self, method : PosterizeMethod);
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
    fn posterize(&mut self, method : PosterizeMethod) {
        let samples = self.pixels().map(|pixel| Vector::from_array(Pixel::into_array(*pixel).map(Convert::convert))).collect::<Vec<_>>();
        match method {
            PosterizeMethod::KMeans { init, cluster_count, } => {
                let (cluster_means, sample_labels, _) = KMeans::new(samples.len(), cluster_count.get()).run(&samples, init, &mut thread_rng());
                for (sample_index, pixel) in self.pixels_mut().enumerate() {
                    let label = sample_labels[sample_index];
                    *pixel = Pixel::from_array(Vector::into_array(cluster_means[label]).map(Convert::convert));
                }
            },
            PosterizeMethod::GaussianMixture { init, cluster_count, } => {
                let (_, cluster_means, _, _, _, _, posteriors) = GaussianMixture::new(samples.len(), cluster_count.get()).run(&samples, init, &mut thread_rng());
                for (sample_index, pixel) in self.pixels_mut().enumerate() {
                    let label = (0..cluster_count.get()).map(|cluster_index| posteriors[cluster_index * samples.len() + sample_index]).position_max_by(f64::total_cmp).unwrap();
                    *pixel = Pixel::from_array(Vector::into_array(cluster_means[label]).map(Convert::convert));
                }
            },
            PosterizeMethod::AgglomerativeSingleLinkage { cluster_count } => {
                let sample_labels = agglomerative_single_linkage(&samples, cluster_count.get());

                // Compute means. This is done for us in the case of k-means clustering cases as
                // that is part of the expectation-maximization algorithm. However, we have to do
                // it ourselves here.
                let mut totals = vec![Vector::zero(); cluster_count.get()];
                let mut counts = vec![0;              cluster_count.get()];
                for (&label, &sample) in std::iter::zip(&sample_labels, &samples) {
                    totals[label] += sample;
                    counts[label] += 1;
                }
                let cluster_means = std::iter::zip(totals, counts).map(|(total, count)| total / count as f64).collect_vec();

                // Same as K-Means clustering
                for (sample_index, pixel) in self.pixels_mut().enumerate() {
                    let label = sample_labels[sample_index];
                    *pixel = Pixel::from_array(Vector::into_array(cluster_means[label]).map(Convert::convert));
                }
            },
        }
    }
}
