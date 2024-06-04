use crate::vector::Vector;
use crate::slice_random_ext::SliceRandomExt as _;

use rayon::prelude::*;
use rand::prelude::*;

use itertools::Itertools;
use std::num::NonZero;

/// Initialization methods for K-Mean Clustering algorithm.
pub enum KMeanInit {
    Llyod,
    KMeanPlusPlus,
}

/// Result from running K-Mean Clustering algorithm.
pub struct KMeanResult<const N: usize> {
    pub labels : Box<[usize]>,
    pub errors : Box<[f32]>,
    pub means : Box<[Vector<f32, N>]>,
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
    init : KMeanInit,
    k : usize,

    samples : Box<[Vector<f32, N>]>,
    labels : Box<[usize]>,
    errors : Box<[f32]>,
    means : Box<[Vector<f32, N>]>,
}

impl<'a, R, const N: usize> KMean<'a, R, N>
where
    R: Rng
{
    /// Constructor.
    ///
    /// The arguments are the same as in [k_mean].
    pub fn new<I>(rng : &'a mut R, init : KMeanInit, k : NonZero<usize>, samples : I) -> Self
    where
        I: IntoIterator<Item = Vector<f32, N>>
    {
        let k = k.into();
        let samples = samples.into_iter().collect::<Box<[_]>>();

        // This is unsafe, and probably invoke undefined behavior in plenty of different ways. But
        // a compiler that does not allow uninitialized POD is pretty stupid in my opionion.
        let labels = unsafe { Box::new_uninit_slice(samples.len()).assume_init() };
        let errors = unsafe { Box::new_uninit_slice(samples.len()).assume_init() };
        let means  = unsafe { Box::new_uninit_slice(k).assume_init() };

        Self { rng, k, init, samples, labels, errors, means, }
    }

    pub fn init(&mut self) {
        self.labels.fill(self.k);
        self.errors.fill(f32::INFINITY);
        match self.init {
            KMeanInit::Llyod => self.means.iter_mut().zip(self.samples.choose_multiple(&mut self.rng, self.k).copied()).for_each(|(mean, choice)| *mean = choice), // Would it be nice if there were to be some Iterator::assign method?
            KMeanInit::KMeanPlusPlus => {
                let mut iter = 0..self.k;

                // 1: Pick initial element and update weights. After that our weights array should
                //    be initialized and usable.
                if let Some(i) = iter.next() {
                    let mean = self.samples.choose(&mut self.rng).copied().unwrap();
                    self.means[i] = mean;
                    self.errors.iter_mut().zip(self.samples.iter().copied()).for_each(|(error, sample)| *error = error.min((mean - sample).squared_length()));
                }

                // 2: Pick other elements and update weights.
                for i in iter.by_ref() {
                    if let Ok(mean) = self.samples.choose_with_weights(&mut self.rng, self.errors.as_ref()).copied() {
                        self.means[i] = mean;
                        self.errors.iter_mut().zip(self.samples.iter().copied()).for_each(|(error, sample)| *error = error.min((mean - sample).squared_length()));
                    } else {
                        break
                    }
                }

                // 3: It is possible that we break out of the previous loop early if our weights
                //    array is malformed in some way. For example, if there all samples share the
                //    same value, the weights array would be zeroed upon picking the first sample,
                //    which is obviously invalid (since the probability of picking each element
                //    would be 0/(0+0...+0)). In that case, we fall back to picking elemenet
                //    normally with replacement.
                for i in iter.by_ref() {
                    let mean = self.samples.choose(&mut self.rng).copied().unwrap();
                    self.means[i] = mean;
                    self.errors.iter_mut().zip(self.samples.iter().copied()).for_each(|(error, sample)| *error = error.min((mean - sample).squared_length()));
                }
            },
        }
    }

    /// Update label and error for each sample to index of nearest cluster and squared distance to
    /// such cluster.
    ///
    /// Return if any label have changed. This could be used as stopping condition.
    pub fn update_labels_and_errors(&mut self) -> bool {
        (self.samples.as_ref(), self.labels.as_mut(), self.errors.as_mut())
            .into_par_iter()
            .map(|(sample, label, error)| {
                let old_label = *label;
                let new_label = self.means
                    .iter()
                    .map(|mean| (*mean - *sample).squared_length())
                    .position_min_by(f32::total_cmp).unwrap();

                *label = new_label;
                *error = (self.means[*label] - *sample).squared_length();

                new_label != old_label
            })
            .reduce(|| false, |a, b| a || b)
    }

    /// Update mean for each cluster by averaging positions of all samples in that cluster.
    pub fn update_means(&mut self) {
        let init_totals = || vec![Vector::<f32, N>::default(); self.means.len()].into_boxed_slice();
        let init_counts = || vec![usize::default();            self.means.len()].into_boxed_slice();
        let init = || (init_totals(), init_counts());

        let (totals, counts) = (self.samples.as_ref(), self.labels.as_ref())
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

        for (mean, total, count) in itertools::izip!(self.means.as_mut(), totals.as_ref(), counts.as_ref()) {
            *mean = if *count != 0 {
                *total / *count as f32
            } else {
                match self.init {
                    KMeanInit::Llyod         =>                                                                                                 self.samples.choose(&mut self.rng).copied().unwrap(),
                    KMeanInit::KMeanPlusPlus => self.samples.choose_with_weights(&mut self.rng, self.errors.iter()).copied().unwrap_or_else(|_| self.samples.choose(&mut self.rng).copied().unwrap()),
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
pub fn k_mean<R, const N: usize, I>(rng : &mut R, init : KMeanInit, k : NonZero<usize>, samples : I) -> KMeanResult<N>
where
    R: Rng,
    I: IntoIterator<Item = Vector<f32, N>>
{
    let mut state = KMean::new(rng, init, k, samples);
    state.init();
    while state.update_labels_and_errors() {
        state.update_means();
    }
    state.finish()
}

