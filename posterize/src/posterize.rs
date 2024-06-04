use crate::Convert;
use crate::Pixel;
use crate::Image;

use cluster::vector::*;
use cluster::k_means::*;

use rand::prelude::*;
use std::num::NonZero;

/// Posterize an image.
pub trait Posterize {
    fn posterize(&mut self, k : NonZero<usize>, init : KMeanInit);
}

/// Implementation of [Posterize] trait for images.
///
/// This is done by applying the k-mean-clustering algorithm with parameter k and replacing each
/// pixel with the mean value of assigned cluster.
///
/// The last trait bound is a work-around for the inability to specify trait bounds on generic
/// associated constants. See [issue #104400](https://github.com/rust-lang/rust/issues/104400).
impl<I, P, C> Posterize for I
where
    I: Image<Pixel = P>,
    P: Pixel<Component = C>,
    C: Convert<f32>, f32: Convert<C>,
    [P::Component; P::COMPONENT_COUNT] : ,
{
    fn posterize(&mut self, k : NonZero<usize>, init : KMeanInit) {
        let samples = self.pixels().map(|pixel| Vector::from_array(Pixel::into_array(*pixel).map(Convert::convert)));
        let kmean = k_mean(&mut thread_rng(), init, k, samples);
        for (pixel, label) in std::iter::zip(self.pixels_mut(), kmean.labels.iter()) {
            *pixel = Pixel::from_array(Vector::into_array(kmean.means[*label]).map(Convert::convert));
        }
    }
}
