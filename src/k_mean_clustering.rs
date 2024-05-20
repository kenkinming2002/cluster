use crate::Vector;

use rand::prelude::*;
use rayon::prelude::*;

use thread_local::ThreadLocal;

use std::cell::Cell;

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

pub fn k_mean_clustering<const N: usize>(values : &[Vector<N>], k : NonZero<usize>) -> KMeanClustering<N> {
    let mut labels : Vec<usize> = vec![Default::default(); values.len()];

    let mut means  : Vec<Vector<N>>                    = (0..k.get()).map(|_| random_vector()).collect();
    let mut totals : Vec<ThreadLocal<Cell<Vector<N>>>> = (0..k.get()).map(|_| Default::default()).collect();
    let mut counts : Vec<ThreadLocal<Cell<usize>>>     = (0..k.get()).map(|_| Default::default()).collect();

    loop {
        // 0: Reset
        std::iter::zip(&mut totals, &mut counts).par_bridge().for_each(|(total, count)| {
            total.iter_mut().for_each(|total| *total = Default::default());
            count.iter_mut().for_each(|count| *count = Default::default());
        });

        // 1: Update labels
        let mut updated = AtomicBool::new(false);
        std::iter::zip(&mut labels, values).par_bridge().for_each(|(label, value)| {
            let new_label = means.iter()
                .map(|mean| (*mean - *value).length_squared())
                .enumerate()
                .min_by(|(_, distance_squared1), (_, distance_squared2)| distance_squared1.total_cmp(distance_squared2))
                .map(|(index, _)| index)
                .unwrap();

            totals[new_label].get_or_default().update(|total| total + *value);
            counts[new_label].get_or_default().update(|count| count + 1);

            if *label != new_label {
                *label = new_label;
                updated.store(true, Ordering::Relaxed);
            }
        });

        // 2: Update means
        std::iter::zip(&mut means, std::iter::zip(&mut totals, &mut counts)).par_bridge().for_each(|(mean, (total, count))| {
            let total = total.iter_mut().map(|total| total.get()).sum::<Vector<N>>();
            let count = count.iter_mut().map(|count| count.get()).sum::<usize>();
            *mean = if count != 0 {
                total / count as f32
            } else {
                random_vector()
            }
        });

        // 3: Check
        if !*updated.get_mut() {
            break KMeanClustering { labels, means }
        }
    }
}

