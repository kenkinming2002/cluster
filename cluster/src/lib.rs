#![allow(clippy::needless_range_loop)]
#![allow(clippy::map_flatten)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::type_complexity)]

pub mod init;
pub mod k_means;
pub mod gaussian_mixture;
pub mod agglomerative_single_linkage;
pub mod affinity_propagation;
pub mod dbscan;

mod slice_random_ext;
mod disjoint_set;
