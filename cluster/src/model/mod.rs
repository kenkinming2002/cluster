//! Implementation of various clustering algorithms.

#[derive(Debug, Clone, Copy)]
pub enum ClusterModel {
    KMeans,
    GaussianMixture,
}

pub mod init;
pub mod k_means;
pub mod gaussian_mixture;
