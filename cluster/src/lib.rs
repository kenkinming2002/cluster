#![feature(generic_nonzero)]
#![feature(new_uninit)]
#![feature(let_chains)]

//! Implementation of various clustering algorithms.

pub mod vector;
pub mod k_means;

mod slice_random_ext;
