//! Naive implemetation of hierarchical agglomerative clustering.
//!
//! This implementation focus on extensiblility rather than optimal performance. For more efficient
//! implementation of specific linkage criterion, see [super::slink] and [super::clink].

pub fn naive<D>(sample_count : usize, cluster_count : usize, mut distance: D) -> Vec<Vec<usize>>
where
    D: FnMut(&[usize], &[usize]) -> f64
{
    let mut clusters = (0..sample_count).map(|index| vec![index]).collect::<Vec<_>>();
    while clusters.len() > cluster_count {
        let mut min_indices = Default::default();
        let mut min_distance = f64::INFINITY;
        for i in 0..clusters.len() {
            for j in i+1..clusters.len() {
                let distance = distance(&clusters[i], &clusters[j]);
                if min_distance > distance {
                    min_distance = distance;
                    min_indices = (i, j);
                }
            }
        }

        let cluster = clusters.swap_remove(min_indices.1);
        clusters[min_indices.0].extend_from_slice(&cluster);
    }
    clusters
}

pub fn single_linkage<D>(mut distance : D) -> impl FnMut(&[usize], &[usize]) -> f64
where
    D: FnMut(usize, usize) -> f64
{
    move |cluster1, cluster2| {
        itertools::Itertools::cartesian_product(cluster1.into_iter(), cluster2.into_iter())
            .map(|(&item1, &item2)| distance(item1, item2))
            .min_by(|a, b| f64::partial_cmp(a, b).unwrap())
            .unwrap()
    }
}

pub fn complete_linkage<D>(mut distance : D) -> impl FnMut(&[usize], &[usize]) -> f64
where
    D: FnMut(usize, usize) -> f64
{
    move |cluster1, cluster2| {
        itertools::Itertools::cartesian_product(cluster1.into_iter(), cluster2.into_iter())
            .map(|(&item1, &item2)| distance(item1, item2))
            .max_by(|a, b| f64::partial_cmp(a, b).unwrap())
            .unwrap()
    }
}

pub fn average_linkage<D>(mut distance : D) -> impl FnMut(&[usize], &[usize]) -> f64
where
    D: FnMut(usize, usize) -> f64
{
    move |cluster1, cluster2| {
        let (total, count) = itertools::Itertools::cartesian_product(cluster1.into_iter(), cluster2.into_iter())
            .map(|(&item1, &item2)| distance(item1, item2))
            .fold((0.0, 0), |(acc_total, acc_count), distance| (acc_total + distance, acc_count + 1));

        total / count as f64
    }
}

