use crate::Vector;
use crate::Counter;

use itertools::Itertools;
use rayon::prelude::*;

use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicBool;

use std::num::NonZero;

struct Sample<const N: usize> {
    value : Vector<N>,
    label : usize,
}

struct Cluster<const N: usize> {
    mean : Vector<N>,
    total : Counter<Vector<N>>,
    count : Counter<usize>,
}

pub struct KMeanClusteringState<const N: usize, F> {
    init : F,
    samples : Vec<Sample<N>>,
    clusters : Vec<Cluster<N>>,
}

pub enum KMeanClusteringResult<const N: usize, F> {
    Incomplete(KMeanClusteringState<N, F>),
    Done(KMeanClusteringState<N, F>),
}

pub struct Labels<'a, const N: usize, F>(&'a KMeanClusteringState<N, F>);
pub struct Means<'a, const N: usize, F>(&'a KMeanClusteringState<N, F>);

impl<const N: usize, F> KMeanClusteringState<N, F> {
    pub fn new<I>(init : F, k : NonZero<usize>, values : I) -> Self
    where
        F: Fn() -> Vector<N> + Sync,
        I: IntoIterator<Item = Vector<N>>,
    {
        let clusters = (0..k.into()).map(|_| Cluster { mean : init(), total : Default::default(), count : Default::default(), }).collect::<Vec<_>>();
        let samples = values.into_iter().map(|value| Sample { value, label : clusters.len(), }).collect();
        Self { init, samples, clusters, }
    }

    pub fn update(mut self) -> KMeanClusteringResult<N, F>
    where
        F: Fn() -> Vector<N> + Sync,
    {
        let done = AtomicBool::new(true);

        // 1: Update sample labels
        self.samples.par_iter_mut().for_each(|sample| {
            let label = self.clusters.iter()
                .map(|cluster| (cluster.mean - sample.value).length_squared()) // Compute squared distance to each cluster
                .position_min_by(f32::total_cmp).unwrap();                     // Get index of minimum element

            self.clusters[label].total.add(sample.value);
            self.clusters[label].count.add(1);
            if sample.label != label {
                sample.label = label;
                done.store(false, Ordering::Relaxed);
            }
        });

        if done.into_inner() {
            return KMeanClusteringResult::Done(self)
        }

        // 2: Update cluster means
        self.clusters.par_iter_mut().for_each(|cluster| {
            let total = cluster.total.sum::<Vector<N>>();
            let count = cluster.count.sum::<usize>();
            cluster.mean = if count != 0 {
                total / count as f32
            } else {
                (self.init)()
            };
        });

        KMeanClusteringResult::Incomplete(self)
    }

    pub fn run(mut self) -> Self
    where
        F: Fn() -> Vector<N> + Sync,
    {
        loop {
            self = match self.update() {
                KMeanClusteringResult::Incomplete(this) => this,
                KMeanClusteringResult::Done(this) => break this,
            }
        }
    }

    pub fn labels(&self) -> Labels<'_, N, F> {
        Labels(self)
    }

    pub fn means(&self) -> Means<'_, N, F> {
        Means(self)
    }
}

impl<const N: usize, F> std::ops::Index<usize> for Labels<'_, N, F> {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0.samples[index].label
    }
}

impl<const N: usize, F> std::ops::Index<usize> for Means<'_, N, F> {
    type Output = Vector<N>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0.clusters[index].mean
    }
}

impl<const N: usize, F> IntoIterator for Labels<'_, N, F> {
    type Item = usize;
    type IntoIter = impl Iterator<Item = Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.samples.iter().map(|sample| sample.label)
    }
}

impl<const N: usize, F> IntoIterator for Means<'_, N, F> {
    type Item = Vector<N>;
    type IntoIter = impl Iterator<Item = Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.clusters.iter().map(|cluster| cluster.mean)
    }
}

