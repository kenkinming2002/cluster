#![feature(new_uninit)]
#![feature(coroutines)]
#![feature(iter_from_coroutine)]
#![feature(type_alias_impl_trait)]
#![feature(test)]
#![feature(vec_push_within_capacity)]

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
