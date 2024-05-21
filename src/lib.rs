#![allow(incomplete_features)]
#![feature(cell_update)]
#![feature(generic_nonzero)]
#![feature(generic_const_exprs)]
#![feature(impl_trait_in_assoc_type)]

mod vector;
mod counter;
mod k_mean_clustering;

mod convert;
mod pixel;
mod image;
mod posterize;

use vector::Vector;
use counter::Counter;
use k_mean_clustering::KMeanClustering;

pub use convert::Convert;
pub use pixel::Pixel;
pub use image::Image;
pub use posterize::posterize;

