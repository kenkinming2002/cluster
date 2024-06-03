use crate::vector::Vector;

use rayon::prelude::*;

use rand::prelude::*;
use rand::distributions::WeightedIndex;

use itertools::Itertools;

use std::num::NonZero;

pub enum KMeanInit {
    Llyod,
    KMeanPlusPlus,
}

/// Result from running K-Mean Clustering algorithm.
pub struct KMeanResult<const N: usize> {
    pub means  : Box<[Vector<f32, N>]>,
    pub labels : Box<[usize]>,
}

pub fn k_means<R, const N: usize>(rng : &mut R, samples : &[Vector<f32, N>], k : NonZero<usize>, init : KMeanInit) -> KMeanResult<N>
where
    R: Rng
{
    match init {
        KMeanInit::Llyod => k_means_llyod(rng, samples, k),
        KMeanInit::KMeanPlusPlus => k_means_plus_plus(rng, samples, k),
    }
}

/// K-Mean Clustering algorithm with llyod initialization.
pub fn k_means_llyod<R, const N: usize>(rng : &mut R, samples : &[Vector<f32, N>], k : NonZero<usize>) -> KMeanResult<N>
where
    R: Rng
{
    let k = k.into();

    // Initialize label for each samples and mean for each clusters.
    //
    // We use k as initial label for each sample. This is such that we always consider labels to
    // have been changed when we update labels for the first time since index of the nearest
    // cluster can only be in the range 0..k.
    //
    // As specified, means is initialized using lloyd initialization method, which is just a fancy
    // way of saying selecting k samples at random.
    let mut means = samples.choose_multiple(rng, k).copied().collect::<Box<[_]>>();
    let mut labels = std::iter::repeat_n(k, samples.len()).collect::<Box<[_]>>();
    loop {
        // 1: Update labels
        let updated =
        {
            let samples = samples.par_iter();
            let labels = labels.par_iter_mut();
            (samples, labels)
                .into_par_iter()
                .map(|(sample, label)| {
                    let old_label = *label;
                    let new_label = means
                        .iter()
                        .map(|mean| (*mean - *sample))
                        .map(|distance| Vector::dot(distance, distance))
                        .position_min_by(f32::total_cmp).unwrap();

                    *label = new_label;
                    new_label != old_label
                })
                .reduce(|| false, |a, b| a || b)
        };

        // 2: Check if labels has changed
        if !updated {
            break KMeanResult { means, labels, }
        }

        // 3: Update means - Compute totals and counts
        let (totals, counts) =
        {
            let init_totals = || vec![Vector::<f32, N>::default(); means.len()].into_boxed_slice();
            let init_counts = || vec![usize::default();            means.len()].into_boxed_slice();
            let init = || (init_totals(), init_counts());

            let values = samples.par_iter();
            let labels = labels.par_iter();
            (values, labels)
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
                })
        };

        // 4: Update means - Divide totals by counts
        {
            let means = means.iter_mut();
            let totals = totals.iter();
            let counts = counts.iter();
            for (mean, total, count) in itertools::izip!(means, totals, counts) {
                *mean = if *count != 0 {
                    *total / *count as f32
                } else {
                    *samples.choose(rng).unwrap()
                }

            }
        }
    }
}

/// K-Mean Clustering algorithm with k-mean++ initialization.
pub fn k_means_plus_plus<R, const N: usize>(rng : &mut R, samples : &[Vector<f32, N>], k : NonZero<usize>) -> KMeanResult<N>
where
    R: Rng
{
    let k = k.into();

    // Initialize label for each samples and mean for each clusters.
    //
    // We use k as initial label for each sample. This is such that we always consider labels to
    // have been changed when we update labels for the first time since index of the nearest
    // cluster can only be in the range 0..k.
    //
    // As specified, cluster means is initialized using k-mean++ initialization method. This means
    // that each cluster mean is picked from samples with a weight that is proportional to its
    // squared distance to the nearest cluster that has been picked. The initial cluster is picked
    // at random.
    //
    // To do so *efficiently*, we use an errors array to keep track of the squared distance from
    // each sample to their nearest cluster and update them progressively as more cluster are
    // picked.
    //
    // The errors array is also updated later on when we are updating label. This is useful in case
    // we want to pick cluster mean again if a cluster has `died' in the sense that no sample is
    // classified to that cluster.
    let mut labels = std::iter::repeat_n(k, samples.len()).collect::<Box<[_]>>();
    let mut errors = vec![f32::INFINITY; samples.len()].into_boxed_slice();
    let mut means = (0..k).map(|i| {
        let mean = match i {
            0 => *samples.choose(rng).unwrap(),
            _ => match WeightedIndex::new(errors.iter().copied()) {
                Ok(dist) => samples[dist.sample(rng)],
                Err(_) => *samples.choose(rng).unwrap(),
            },
        };
        errors.par_iter_mut().zip(samples).for_each(|(error, sample)| *error = f32::min(*error, (mean - *sample).squared_length()));
        mean
    }).collect::<Box<[_]>>();

    loop {
        // 1: Update labels
        let updated =
        {
            let samples = samples.par_iter();
            let labels = labels.par_iter_mut();
            let errors = errors.par_iter_mut();
            (samples, labels, errors)
                .into_par_iter()
                .map(|(sample, label, error)| {
                    let old_label = *label;
                    let new_label = means
                        .iter()
                        .map(|mean| (*mean - *sample).squared_length())
                        .position_min_by(f32::total_cmp).unwrap();

                    *label = new_label;
                    *error = (means[*label] - *sample).squared_length();

                    new_label != old_label
                })
                .reduce(|| false, |a, b| a || b)
        };

        // 2: Check if labels has changed
        if !updated {
            break KMeanResult { means, labels, }
        }

        // 3: Update means - Compute totals and counts
        let (totals, counts) =
        {
            let init_totals = || vec![Vector::<f32, N>::default(); means.len()].into_boxed_slice();
            let init_counts = || vec![usize::default();            means.len()].into_boxed_slice();
            let init = || (init_totals(), init_counts());

            let values = samples.par_iter();
            let labels = labels.par_iter();
            (values, labels)
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
                })
        };

        // 4: Update means - Divide totals by counts
        {
            let means = means.iter_mut();
            let totals = totals.iter();
            let counts = counts.iter();
            for (mean, total, count) in itertools::izip!(means, totals, counts) {
                *mean = if *count != 0 {
                    *total / *count as f32
                } else {
                    match WeightedIndex::new(errors.iter().copied()) {
                        Ok(dist) => samples[dist.sample(rng)],
                        Err(_) => *samples.choose(rng).unwrap(),
                    }
                }
            }
        }
    }
}
