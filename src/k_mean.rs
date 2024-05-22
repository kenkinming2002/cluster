use std::num::NonZero;
use std::ops::AddAssign;

use rand::prelude::*;
use rayon::prelude::*;
use itertools::Itertools;

pub struct KMean {
    sample_count     : usize,
    sample_dimension : usize,
    cluster_count    : usize,

    values : Box<[f32]>,
    labels : Box<[usize]>,
    means  : Box<[f32]>,
}

impl KMean {
    pub fn new(sample_count : usize, sample_dimension : usize, cluster_count : NonZero<usize>, values : impl Into<Box<[f32]>>) -> Self {
        let cluster_count = cluster_count.get();
        let values = values.into();

        assert!(values.len() == sample_count * sample_dimension);

        let labels = unsafe { Box::new_uninit_slice(sample_count).assume_init() };
        let means  = unsafe { Box::new_uninit_slice(cluster_count * sample_dimension).assume_init() };

        Self { sample_count, sample_dimension, cluster_count, values, labels, means, }
    }

    pub fn init_llyod<R>(mut self, rng : &mut R) -> Self
    where
        R: Rng + ?Sized,
    {
        self.labels.fill(self.cluster_count);
        self.means.chunks_exact_mut(self.sample_dimension).for_each(|mean| {
            mean.copy_from_slice(self.values.chunks_exact(self.sample_dimension).choose(rng).unwrap());
        });
        self
    }

    pub fn run(mut self) -> Self {
        loop {
            // 1: Update labels
            let values = self.values.par_chunks_exact(self.sample_dimension);
            let labels = self.labels.par_iter_mut();
            let updated = (values, labels)
                .into_par_iter()
                .map(|(value, label)| {
                    let old_label = *label;
                    let new_label = self.means
                        .chunks_exact(self.sample_dimension)
                        .map(|mean| std::iter::zip(mean, value).map(|(x, y)| *x - *y).map(|x| x * x).sum()) // Get squared distance to each cluster
                        .position_min_by(f32::total_cmp).unwrap();                                          // Get index of minimum element

                    *label = new_label;
                    new_label != old_label
                })
                .reduce(|| false, |a, b| a || b);

            if !updated {
                break self
            }

            // 2: Update means
            let init_totals = || vec![0.0; self.cluster_count * self.sample_dimension].into_boxed_slice();
            let init_counts = || vec![0; self.cluster_count].into_boxed_slice();
            let init = || (init_totals(), init_counts());

            let values = self.values.par_chunks_exact(self.sample_dimension);
            let labels = self.labels.par_iter();
            let (totals, counts) = (values, labels)
                .into_par_iter()
                .fold(init, |(mut totals, mut counts), (value, label)| {

                    let total = &mut totals[self.sample_dimension * *label..self.sample_dimension * (*label + 1)];
                    let count = &mut counts[*label];

                    total.iter_mut().zip(value).for_each(|(x, y)| *x += *y);
                    count.add_assign(1);

                    (totals, counts)
                })
                .reduce(init, |(mut totals1, mut counts1), (totals2, counts2)| {
                    {
                        let totals1 = totals1.chunks_exact_mut(self.sample_dimension);
                        let totals2 = totals2.chunks_exact(self.sample_dimension);

                        let counts1 = counts1.iter_mut();
                        let counts2 = counts2.iter();

                        std::iter::zip(totals1, totals2).for_each(|(total1, total2)| std::iter::zip(total1, total2).for_each(|(x, y)| *x += *y));
                        std::iter::zip(counts1, counts2).for_each(|(count1, count2)| *count1 += *count2);
                    }
                    (totals1, counts1)
                });

            let means = self.means.chunks_exact_mut(self.sample_dimension);
            let totals = totals.chunks_exact(self.sample_dimension);
            let counts = counts.iter();
            itertools::izip!(means, totals, counts).for_each(|(mean, total, count)| std::iter::zip(mean, total).for_each(|(x, y)| *x = *y / *count as f32));
        }
    }

    pub fn sample_count(&self) -> usize {
        self.sample_count
    }

    pub fn sample_dimension(&self) -> usize {
        self.sample_dimension
    }

    pub fn cluster_count(&self) -> usize {
        self.cluster_count
    }

    pub fn values(&self) -> &[f32] {
        &self.values
    }

    pub fn labels(&self) -> &[usize] {
        &self.labels
    }

    pub fn means(&self) -> &[f32] {
        &self.means
    }
}
