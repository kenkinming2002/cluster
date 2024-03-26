#![feature(cell_update)]

pub mod vec;
pub use vec::Vector;

use rand::prelude::*;
use rayon::prelude::*;

use thread_local::ThreadLocal;

use std::cell::Cell;

use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicBool;

#[derive(Debug, Default)]
struct Point<const N: usize> {
    position : Vector<N>,
    label : usize,
}

#[derive(Debug, Default)]
struct Cluster<const N: usize> {
    mean : Vector<N>,
    total : ThreadLocal<Cell<Vector<N>>>,
    count : ThreadLocal<Cell<usize>>,
}

#[derive(Debug, Default)]
pub struct KMeanClusteringState<const N: usize> {
    points : Vec<Point<N>>,
    clusters : Vec<Cluster<N>>,
    updated : AtomicBool,
}

impl<const N: usize> KMeanClusteringState<N> {
    pub fn new<P, M>(positions : P, means : M) -> Self
    where
        P: IntoIterator<Item = Vector<N>>,
        M: IntoIterator<Item = Vector<N>>,
    {
        Self {
            points   : positions.into_iter().map(|position| Point   { position, ..Default::default() }).collect(),
            clusters : means    .into_iter().map(|mean|     Cluster { mean,     ..Default::default() }).collect(),
            ..Default::default()
        }
    }

    fn reset(&mut self) {
        for cluster in self.clusters.iter_mut() {
            cluster.total.clear();
            cluster.count.clear();
        }
        self.updated = AtomicBool::from(false);
    }

    fn update_label(&mut self) {
        self.points.par_iter_mut().for_each(|point| {
            let new_label = self.clusters.iter()
                                         .map(|cluster| (cluster.mean - point.position).length_squared())
                                         .enumerate()
                                         .min_by(|(_, distance_squared1), (_, distance_squared2)| distance_squared1.total_cmp(distance_squared2))
                                         .map(|(index, _)| index)
                                         .unwrap();

            self.clusters[new_label].total.get_or_default().update(|total| total + point.position);
            self.clusters[new_label].count.get_or_default().update(|count| count + 1);
            if point.label != new_label {
                point.label = new_label;
                self.updated.store(true, Ordering::Relaxed);
            }
        });
    }

    fn update_mean(&mut self) {
        for cluster in self.clusters.iter_mut() {
            let total = cluster.total.iter_mut().map(|total| total.get()).sum::<Vector<N>>();
            let count = cluster.count.iter_mut().map(|count| count.get()).sum::<usize>();
            cluster.mean = if count != 0 {
                total / count as f32
            } else {
                Vector([(); N].map(|_| thread_rng().gen_range(0.0..255.0)))
            }
        }
    }

    pub fn step(&mut self) -> bool {
        self.reset();
        self.update_label();
        self.update_mean();

        *self.updated.get_mut()
    }

    pub fn labels(&self) -> impl Iterator<Item = usize> + '_ {
        self.points.iter().map(|point| point.label)
    }

    pub fn means(&self) -> impl Iterator<Item = Vector<N>> + '_ {
        self.clusters.iter().map(|cluster| cluster.mean)
    }
}

