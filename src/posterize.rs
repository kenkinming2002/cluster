use crate::Convert;
use crate::Pixel;
use crate::Image;

use crate::Vector;
use crate::KMeanClusteringState;

use rand::prelude::*;

use std::num::NonZero;

fn random_vector<const N: usize>() -> Vector<N> {
    Vector([(); N].map(|_| thread_rng().gen_range(0.0..255.0)))
}

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
    let values = image.pixels().map(|x| Vector(x.into_array().map(|x| x.convert())));

    let state = KMeanClusteringState::new(random_vector, k, values);
    let state = state.run();

    let labels = state.labels();
    let means  = state.means();
    for (pixel, label) in std::iter::zip(image.pixels_mut(), labels) {
        *pixel = P::from_array(means[label].0.map(|x| x.convert()));
    }
}

