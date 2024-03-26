pub mod vec;
pub use vec::Vector;

use rand::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
struct Point<const N: usize> {
    position : Vector<N>,
    label : usize,
}

#[derive(Debug, Clone, Copy, Default)]
struct Cluster<const N: usize> {
    mean : Vector<N>,
    total : Vector<N>,
    count : usize,
}

#[derive(Debug, Clone, Default)]
pub struct KMeanClusteringState<const N: usize> {
    points : Vec<Point<N>>,
    clusters : Vec<Cluster<N>>,
    updated : bool,
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
            cluster.total = Default::default();
            cluster.count = 0;
        }
        self.updated = false;
    }

    fn update_label(&mut self) {
        for point in self.points.iter_mut() {
            let new_label = self.clusters.iter()
                                         .map(|cluster| (cluster.mean - point.position).length_squared())
                                         .enumerate()
                                         .min_by(|(_, distance_squared1), (_, distance_squared2)| distance_squared1.total_cmp(distance_squared2))
                                         .map(|(index, _)| index)
                                         .unwrap();

            self.clusters[new_label].total = self.clusters[new_label].total + point.position;
            self.clusters[new_label].count = self.clusters[new_label].count + 1;

            if point.label != new_label {
                point.label = new_label;
                self.updated = true;
            }
        }
    }

    fn update_mean(&mut self) {
        for cluster in self.clusters.iter_mut() {
            if cluster.count != 0 {
                cluster.mean = cluster.total / cluster.count as f32;
            } else {
                let mut rng = thread_rng();
                for i in 0..N {
                    cluster.mean.0[i] = rng.gen_range(0.0..=255.0);
                }
            }
        }
    }

    pub fn step(&mut self) -> bool {
        self.reset();
        self.update_label();
        self.update_mean();
        self.updated
    }

    pub fn labels(&self) -> impl Iterator<Item = usize> + '_ {
        self.points.iter().map(|point| point.label)
    }

    pub fn means(&self) -> impl Iterator<Item = Vector<N>> + '_ {
        self.clusters.iter().map(|cluster| cluster.mean)
    }
}

