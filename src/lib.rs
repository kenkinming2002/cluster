#![allow(incomplete_features)]
#![feature(generic_nonzero)]
#![feature(generic_const_exprs)]
#![feature(new_uninit)]

mod convert;
mod pixel;
mod image;

mod k_mean;
mod posterize;

pub use convert::Convert;
pub use pixel::Pixel;
pub use image::Image;

pub use k_mean::KMean;
pub use posterize::posterize;

