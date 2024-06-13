#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(new_uninit)]

mod convert;
mod pixel;
mod image;
mod posterize;

pub use convert::Convert;
pub use pixel::Pixel;
pub use image::Image;
pub use posterize::Posterize;
pub use posterize::PosterizeMethod;

