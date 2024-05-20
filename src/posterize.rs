use crate::Convert;
use crate::Pixel;
use crate::Image;

use crate::Vector;
use crate::KMeanClustering;

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
    let means  = (0..k.into()).map(|_| random_vector());

    let mut k_mean_clustering = KMeanClustering::new(values, means);
    k_mean_clustering.run(|| random_vector());

    let labels = k_mean_clustering.labels();
    let means  = k_mean_clustering.means();
    for (pixel, label) in std::iter::zip(image.pixels_mut(), labels) {
        *pixel = P::from_array(means[label].0.map(|x| x.convert()));
    }
}

