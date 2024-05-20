#![allow(incomplete_features)]
#![feature(cell_update)]
#![feature(generic_nonzero)]
#![feature(generic_const_exprs)]

mod vec;
mod k_mean_clustering;

mod convert;
mod pixel;
mod image;
mod posterize;

use vec::Vector;
use k_mean_clustering::KMeanClustering;
use k_mean_clustering::k_mean_clustering;

pub use convert::Convert;
pub use pixel::Pixel;
pub use image::Image;
pub use posterize::posterize;

