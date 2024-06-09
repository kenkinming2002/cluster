mod vector;
mod matrix;

mod inner_product;
mod outer_product;

mod mse;

mod multivariate_gaussian;

pub use vector::Vector;
pub use matrix::Matrix;

pub use inner_product::InnerProduct;
pub use outer_product::OuterProduct;

pub use mse::MseIteratorExt;

pub use multivariate_gaussian::MultivariateGaussian;
