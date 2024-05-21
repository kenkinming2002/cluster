use crate::Vector;
use crate::Counter;

use itertools::Itertools;
use rayon::prelude::*;

use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicBool;

struct Sample<const N: usize> {
    value : Vector<N>,
    label : usize,
}

struct Cluster<const N: usize> {
    mean : Vector<N>,
    total : Counter<Vector<N>>,
    count : Counter<usize>,
}

pub struct KMeanClustering<const N: usize> {
    samples : Vec<Sample<N>>,
    clusters : Vec<Cluster<N>>,
    done : AtomicBool,
}

pub struct Labels<'a, const N: usize>(&'a KMeanClustering<N>);
pub struct Means<'a, const N: usize>(&'a KMeanClustering<N>);

impl<const N: usize> KMeanClustering<N> {
    pub fn new<VS, MS>(values : VS, means : MS) -> Self
    where
        VS: IntoIterator<Item = Vector<N>>,
        MS: IntoIterator<Item = Vector<N>>,
    {
        let clusters = means.into_iter().map(|mean| Cluster { mean, total : Default::default(), count : Default::default(), } ).collect::<Vec<_>>();
        let samples = values.into_iter().map(|value| Sample { value, label : clusters.len(), }).collect();
        Self { samples, clusters, done : AtomicBool::new(false), }
    }

    pub fn update_label(&mut self) {
        self.samples.par_iter_mut().for_each(|sample| {
            let label = self.clusters.iter()
                .map(|cluster| (cluster.mean - sample.value).length_squared()) // Compute squared distance to each cluster
                .position_min_by(f32::total_cmp).unwrap();                     // Get index of minimum element

            self.clusters[label].total.add(sample.value);
            self.clusters[label].count.add(1);
            if sample.label != label {
                sample.label = label;
                self.done.store(false, Ordering::Relaxed);
            }
        });

    }

    pub fn update_mean<F>(&mut self, init : F)
    where
        F: Fn() -> Vector<N> + Sync
    {
        self.clusters.par_iter_mut().for_each(|cluster| {
            let total = cluster.total.sum::<Vector<N>>();
            let count = cluster.count.sum::<usize>();
            cluster.mean = if count != 0 {
                total / count as f32
            } else {
                init()
            };
        });
    }

    pub fn update<F>(&mut self, init : F)
    where
        F: Fn() -> Vector<N> + Sync
    {
        self.done = AtomicBool::new(true);
        self.update_label();
        self.update_mean(init);
    }

    pub fn run<F>(&mut self, init : F)
    where
        F: Fn() -> Vector<N> + Sync
    {
        while !self.done() {
            self.update(&init);
        }
    }

    pub fn done(&mut self) -> bool {
        std::mem::take(&mut self.done).into_inner()
    }

    pub fn labels(&self) -> Labels<'_, N> {
        Labels(&self)
    }

    pub fn means(&self) -> Means<'_, N> {
        Means(&self)
    }
}

impl<const N: usize> std::ops::Index<usize> for Labels<'_, N> {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0.samples[index].label
    }
}

impl<const N: usize> std::ops::Index<usize> for Means<'_, N> {
    type Output = Vector<N>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0.clusters[index].mean
    }
}

impl<const N: usize> IntoIterator for Labels<'_, N> {
    type Item = usize;
    type IntoIter = impl Iterator<Item = Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.samples.iter().map(|sample| sample.label)
    }
}

impl<const N: usize> IntoIterator for Means<'_, N> {
    type Item = Vector<N>;
    type IntoIter = impl Iterator<Item = Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.clusters.iter().map(|cluster| cluster.mean)
    }
}

