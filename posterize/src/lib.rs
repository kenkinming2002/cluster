//! This crate implement image posterization using various(i.e. 2) clustering algorithms.

pub use cluster::math::Vector;
pub use cluster::model::init::ClusterInit;

use cluster::model::k_means::*;
use cluster::model::gaussian_mixture::*;

use rand::prelude::*;
use clap::Subcommand;
use std::num::NonZero;

/// Enum containing different clustering algorithms that can be used for posterization.
///
/// While there are many more clustering algorithm implemented in the cluster crate, only kmeans
/// and gaussian mixture are supported because other algorithm are computational infeasible (At
/// least in my implementation).
#[derive(Debug, Clone, Copy, Subcommand)]
pub enum PosterizeMethod {
    KMeans { cluster_init : ClusterInit, cluster_count : NonZero<usize>, },
    GaussianMixture { cluster_init : ClusterInit, cluster_count : NonZero<usize>, },
}

impl PosterizeMethod {
    /// Posterize using the specified method.
    ///
    /// Apply the specified clustering algorithm to provided samples and replace each sample with
    /// the center of the cluster it belongs to.
    pub fn posterize<const N: usize>(self, samples : &mut [Vector<N>]) {
        let sample_count = samples.len();
        match self {
            PosterizeMethod::KMeans { cluster_init, cluster_count } => {
                let (means, labels, _) = KMeans::new(samples.len(), cluster_count.into()).run(samples, cluster_init, &mut thread_rng());
                for (index, pixel) in samples.iter_mut().enumerate() {
                    *pixel = means[labels[index]];
                }
            }
            PosterizeMethod::GaussianMixture { cluster_init, cluster_count } => {
                let (_, means, _, _, _, _, posteriors) = GaussianMixture::new(samples.len(), cluster_count.into()).run(samples, cluster_init, &mut thread_rng());
                for (index, pixel) in samples.iter_mut().enumerate() {
                    *pixel = (0..cluster_count.into()).map(|cluster_index| means[cluster_index] * posteriors[cluster_index * sample_count + index]).sum();
                }
            }
        }
    }
}

