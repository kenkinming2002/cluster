use crate::Vector;

use std::num::NonZero;

use rand::prelude::*;
use rayon::prelude::*;
use itertools::Itertools;

pub struct KMean<const N : usize> {
    values : Box<[Vector<N>]>,
    labels : Box<[usize]>,
    means  : Box<[Vector<N>]>,
}

impl<const N : usize> KMean<N> {
    pub fn new(values : impl Into<Box<[Vector<N>]>>, k : NonZero<usize>) -> Self {
        let k = k.into();

        let values = values.into();
        let labels = vec![k; values.len()].into();
        let means = vec![Default::default(); k].into();

        Self { values, labels, means, }
    }

    pub fn init_llyod<R>(mut self, rng : &mut R) -> Self
    where
        R: Rng + ?Sized,
    {
        self.labels.fill(self.means.len());
        self.means.iter_mut().for_each(|mean| *mean = *self.values.choose(rng).unwrap());
        self
    }

    pub fn run(mut self) -> Self {
        loop {
            // 1: Update labels
            let values = self.values.par_iter();
            let labels = self.labels.par_iter_mut();
            let updated = (values, labels)
                .into_par_iter()
                .map(|(value, label)| {
                    let old_label = *label;
                    let new_label = self.means
                        .iter()
                        .map(|mean| (*mean - *value).length_squared()) // Get squared distance to each cluster
                        .position_min_by(f32::total_cmp).unwrap();     // Get index of minimum element

                    *label = new_label;
                    new_label != old_label
                })
                .reduce(|| false, |a, b| a || b);

            if !updated {
                break self
            }

            // 2: Update means
            let init_totals = || vec![Vector::default(); self.means.len()].into_boxed_slice();
            let init_counts = || vec![usize::default();  self.means.len()].into_boxed_slice();
            let init = || (init_totals(), init_counts());

            let values = self.values.par_iter();
            let labels = self.labels.par_iter();
            let (totals, counts) = (values, labels)
                .into_par_iter()
                .fold(init, |(mut totals, mut counts), (value, label)| {
                    totals[*label] = totals[*label] + *value;
                    counts[*label] = counts[*label] + 1;

                    (totals, counts)
                })
                .reduce(init, |(mut totals1, mut counts1), (totals2, counts2)| {
                    std::iter::zip(totals1.iter_mut(), totals2.iter()).for_each(|(total1, total2)| *total1 = *total1 + *total2);
                    std::iter::zip(counts1.iter_mut(), counts2.iter()).for_each(|(count1, count2)| *count1 = *count1 + *count2);

                    (totals1, counts1)
                });

            let means = self.means.iter_mut();
            let totals = totals.iter();
            let counts = counts.iter();
            for (mean, total, count) in itertools::izip!(means, totals, counts) {
                if *count != 0 {
                    *mean = *total / *count as f32;
                }
            }
        }
    }

    pub fn values(&self) -> &[Vector<N>] {
        &self.values
    }

    pub fn labels(&self) -> &[usize] {
        &self.labels
    }

    pub fn means(&self) -> &[Vector<N>] {
        &self.means
    }
}
