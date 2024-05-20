use crate::Vector;
use crate::Counter;

use itertools::Itertools;
use rand::prelude::*;
use rayon::prelude::*;

use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicBool;

use std::num::NonZero;

pub struct KMeanClustering<const N: usize> {
    pub labels : Vec<usize>,
    pub means : Vec<Vector<N>>,
}

fn random_vector<const N: usize>() -> Vector<N> {
    Vector([(); N].map(|_| thread_rng().gen_range(0.0..255.0)))
}

struct Sample<const N: usize> {
    value : Vector<N>,
    label : usize,
}

struct Cluster<const N: usize> {
    mean : Vector<N>,
    total : Counter<Vector<N>>,
    count : Counter<usize>,
}

pub fn k_mean_clustering<const N: usize, I>(values : I, k : NonZero<usize>) -> KMeanClustering<N>
where
    I: IntoIterator<Item = Vector<N>>
{
    let mut clusters = (0..k.get()).map(|_| Cluster { mean : random_vector(), total : Default::default(), count : Default::default(), }).collect::<Vec<_>>();
    let mut samples  = values.into_iter().map(|value| Sample { value, label : 0, }).collect::<Vec<_>>();
    loop {
        let updated = AtomicBool::new(false);

        // 1: Update label for each sample and corresponding counter in each cluster
        samples.par_iter_mut().for_each(|sample| {
            let label = clusters.iter()
                                .map(|cluster| (cluster.mean - sample.value).length_squared()) // Compute squared distance to each cluster
                                .position_min_by(f32::total_cmp).unwrap();                     // Get index of minimum element

            clusters[label].total.add(sample.value);
            clusters[label].count.add(1);
            if sample.label != label {
                sample.label = label;
                updated.store(true, Ordering::Relaxed);
            }
        });

        // 2: Check termination
        if !updated.into_inner() {
            let means = clusters.into_iter().map(|cluster| cluster.mean).collect::<Vec<_>>();
            let labels = samples.into_iter().map(|sample| sample.label).collect::<Vec<_>>();
            break KMeanClustering { labels, means }
        }

        // 3: Update clusters mean
        clusters.par_iter_mut().for_each(|cluster| {
            let total = cluster.total.sum::<Vector<N>>();
            let count = cluster.count.sum::<usize>();
            cluster.mean = if count != 0 {
                total / count as f32
            } else {
                random_vector()
            }
        });
    }
}

