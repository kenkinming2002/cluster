#![feature(coroutines)]
#![feature(iter_from_coroutine)]
#![feature(type_alias_impl_trait)]

pub mod vector;
pub mod matrix;

pub mod inner_product;
pub mod outer_product;

pub mod multivariate_gaussian;
pub mod mse;

pub mod prelude {
    pub use crate::vector::Vector;
    pub use crate::matrix::Matrix;

    pub use crate::inner_product::InnerProduct;
    pub use crate::outer_product::OuterProduct;

    pub use crate::multivariate_gaussian::MultivariateGaussian;
    pub use crate::mse::MseIteratorExt;
}

mod array_zip;
mod permutation;
