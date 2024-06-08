#![feature(generic_nonzero)]
#![feature(new_uninit)]
#![feature(coroutines)]
#![feature(iter_from_coroutine)]
#![feature(type_alias_impl_trait)]

//! Implementation of various clustering algorithms.

pub mod math;

pub mod init;
pub mod k_means;
pub mod gaussian_mixture;

mod array_zip;
mod slice_random_ext;
mod permutation;

#[derive(Debug, Clone, Copy)]
pub enum ClusterModel {
    KMeans,
    GaussianMixture,
}

