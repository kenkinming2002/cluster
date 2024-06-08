use crate::init::ClusterInit;
use crate::math::Vector;
use crate::slice_random_ext::SliceRandomExt as _;

use rayon::prelude::*;
use rand::prelude::*;

use itertools::Itertools;
use std::num::NonZero;

/// Result from running K-Mean Clustering algorithm.
pub struct KMeanResult<const N: usize> {
    pub labels : Vec<usize>,
    pub errors : Vec<f64>,
    pub means : Vec<Vector<N>>,
}

/// Implementation of K-Mean Clustering algorithm.
///
/// This interface allows you to step through the algorithm step-by-step which can be useful for
/// visualization. Otherwise, use [k_mean] instead.
pub struct KMean<'a, R, const N: usize>
where
    R: Rng
{
    rng : &'a mut R,
    init : ClusterInit,

    k : usize,

    samples : Vec<Vector<N>>,

    means : Vec<Vector<N>>,

    labels : Vec<usize>,
    errors : Vec<f64>,
}

impl<'a, R, const N: usize> KMean<'a, R, N>
where
    R: Rng
{
    /// Constructor.
    ///
    /// The arguments are the same as in [k_mean].
    pub fn new<I>(rng : &'a mut R, init : ClusterInit, k : NonZero<usize>, samples : I) -> Self
    where
        I: IntoIterator<Item = Vector<N>>
    {
        let samples = samples.into_iter().collect::<Vec<_>>();

        let n = samples.len();
        let k = k.into();

        // This is unsafe, and probably invoke undefined behavior in plenty of different ways. But
        // a compiler that does not allow uninitialized POD is pretty stupid in my opionion.

        let means = unsafe { Box::new_uninit_slice(k).assume_init().into_vec() };

        let labels = unsafe { Box::new_uninit_slice(n).assume_init().into_vec() };
        let errors = unsafe { Box::new_uninit_slice(n).assume_init().into_vec() };

        Self { rng, k, init, samples, means, labels, errors, }
    }

    pub fn init(&mut self) {
        self.means.copy_from_slice(&self.init.init(self.rng, &self.samples, self.k));
        self.labels.fill(self.k);
        self.errors.fill(f64::INFINITY);
    }

    /// Update label and error for each sample to index of nearest cluster and squared distance to
    /// such cluster.
    ///
    /// Return if any label have changed. This could be used as stopping condition.
    pub fn update_labels_and_errors(&mut self) -> bool {
        self.samples
            .par_iter()
            .zip((&mut self.labels, &mut self.errors).into_par_iter())
            .map(|(sample, (label, error))| {
                let old_label = *label;
                let new_label = self.means
                    .iter()
                    .map(|mean| (*mean - *sample).squared_length())
                    .position_min_by(f64::total_cmp).unwrap();

                *label = new_label;
                *error = (self.means[*label] - *sample).squared_length();

                new_label != old_label
            })
            .reduce(|| false, |a, b| a || b)
    }

    /// Update mean for each cluster by averaging positions of all samples in that cluster.
    pub fn update_means(&mut self) {
        let init_totals = || vec![Vector::<N>::default(); self.means.len()];
        let init_counts = || vec![usize::default();       self.means.len()];
        let init = || (init_totals(), init_counts());

        let (totals, counts) = (&self.samples, &self.labels)
            .into_par_iter()
            .fold(init, |(mut totals, mut counts), (value, label)| {
                totals[*label] += *value;
                counts[*label] += 1;
                (totals, counts)
            })
            .reduce(init, |(mut totals1, mut counts1), (totals2, counts2)| {
                std::iter::zip(totals1.iter_mut(), totals2.iter()).for_each(|(total1, total2)| *total1 += *total2);
                std::iter::zip(counts1.iter_mut(), counts2.iter()).for_each(|(count1, count2)| *count1 += *count2);
                (totals1, counts1)
            });

        for (mean, (total, count)) in self.means.iter_mut().zip(std::iter::zip(&totals, &counts)) {
            *mean = if *count != 0 {
                *total / *count as f64
            } else {
                match self.init {
                    ClusterInit::Llyod         =>                                                                                                 self.samples.choose(&mut self.rng).copied().unwrap(),
                    ClusterInit::KMeanPlusPlus => self.samples.choose_with_weights(&mut self.rng, self.errors.iter()).copied().unwrap_or_else(|_| self.samples.choose(&mut self.rng).copied().unwrap()),
                }
            }
        }
    }

    /// Return the finished result from running the algorithm.
    ///
    /// Note that this **DOES NOT** actually run the algorithm. You **MUST** call methods such as
    /// [KMean::init], [KMean::update_labels_and_errors] and [KMean::update_means] as appropriate
    /// before trying to retrieve the result.
    pub fn finish(self) -> KMeanResult<N> {
        KMeanResult {
            labels : self.labels,
            errors : self.errors,
            means : self.means,
        }
    }
}

/// Implementation of K-Mean Clustering algorithm.
pub fn k_mean<R, const N: usize, I>(rng : &mut R, init : ClusterInit, k : NonZero<usize>, samples : I) -> KMeanResult<N>
where
    R: Rng,
    I: IntoIterator<Item = Vector<N>>
{
    let mut state = KMean::new(rng, init, k, samples);
    state.init();
    while state.update_labels_and_errors() {
        state.update_means();
    }
    state.finish()
}

