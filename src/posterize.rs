use crate::Convert;
use crate::Pixel;
use crate::Image;

use crate::Vector;
use crate::k_mean_clustering;
use crate::KMeanClustering;

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
    // The curse of strong typing means that we have to do stupid things like this.
    let values = image.pixels().map(|x| Vector(x.into_array().map(|x| x.convert()))).collect::<Vec<_>>();
    let KMeanClustering { labels, means } = k_mean_clustering(&values, k);
    for (pixel, label) in std::iter::zip(image.pixels_mut(), labels) {
        *pixel = P::from_array(means[label].0.map(|x| x.convert()));
    }
}

