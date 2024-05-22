use crate::Convert;
use crate::Pixel;
use crate::Image;
use crate::KMean;

use rand::prelude::*;

use std::num::NonZero;

/// Posterize an image.
///
/// This is done by applying the k-mean-clustering algorithm with parameter k and replacing each
/// pixel with the mean value of assigned cluster.
///
/// The last trait bound is a work-around for the inability to specify trait bounds on generic
/// associated constants. See [issue #104400](https://github.com/rust-lang/rust/issues/104400).
pub fn posterize<I, P, C>(image : &mut I, k : NonZero<usize>)
where
    I: Image<Pixel = P>,
    P: Pixel<Component = C>,
    C: Convert<f32>, f32: Convert<C>,
    [P::Component; P::COMPONENT_COUNT] : ,
{
    let mut rng = thread_rng();

    let sample_count = image.width() * image.height();
    let sample_dimension = P::COMPONENT_COUNT;
    let cluster_count = k;

    let values = image.pixels().flat_map(|x| x.into_array()).map(|x| x.convert()).collect::<Vec<f32>>();
    let kmean  = KMean::new(sample_count, sample_dimension, cluster_count, values).init_llyod(&mut rng).run();

    let pixels = image.pixels_mut();
    let labels = kmean.labels().iter();
    for (pixel, label) in std::iter::zip(pixels, labels) {
        let mut means = kmean.means().chunks_exact(sample_dimension);

        let mean = means.nth(*label).unwrap();
        let mean = TryInto::<[_; P::COMPONENT_COUNT]>::try_into(mean).unwrap();
        let mean = mean.map(|x| x.convert());

        *pixel = P::from_array(mean);
    }
}

