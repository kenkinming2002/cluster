//! Implementation of various clustering algorithms.

#[derive(Debug, Clone, Copy)]
pub enum ClusterModel {
    KMeans,
    GaussianMixture,
    AgglomerativeSingleLinkage,
}

pub mod init;
pub mod k_means;
pub mod gaussian_mixture;
pub mod agglomerative;
pub mod affinity_propagation;
