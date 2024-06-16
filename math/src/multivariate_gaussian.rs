use crate::prelude::*;

pub struct MultivariateGaussian<const N: usize> {
    mean : Vector<N>,
    bilinear_form : Matrix<N>,
    normalizing_factor : f64,
}

impl<const N: usize> MultivariateGaussian<N> {
    pub fn new(mean : Vector<N>, covariance : Matrix<N>) -> Self {
        let covariance_inv = covariance.inverse();
        let covariance_det = covariance.determinant();

        let bilinear_form = covariance_inv * -0.5;
        let normalizing_factor = 1.0 / ((2.0 * std::f64::consts::PI).powi(N as i32) * covariance_det).sqrt();

        Self { mean, bilinear_form, normalizing_factor, }
    }

    pub fn sample(&self, point : Vector<N>) -> f64 {
        let displacement = point - self.mean;
        self.bilinear_form.inner_product(displacement).inner_product(displacement).exp() * self.normalizing_factor
    }
}
