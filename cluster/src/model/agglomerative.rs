use crate::math::Vector;
use crate::utils::disjoint_set::DisjointSet;

use std::collections::BinaryHeap;

#[derive(Debug, Copy, Clone)]
struct Edge {
    index1 : usize,
    index2 : usize,
    distance : f64,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for Edge {}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.distance.partial_cmp(&other.distance)?.reverse())
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.total_cmp(&other.distance).reverse()
    }
}

/// Implementation of single-linkage agglomerative clustering.
pub fn agglomerative_single_linkage<const N: usize>(samples : &[Vector<N>], cluster_count : usize) -> Vec<usize> {
    let mut edges = Vec::with_capacity(samples.len() * (samples.len() - 1) / 2);
    for index1 in 0..samples.len() {
        for index2 in index1+1..samples.len() {
            let _ = edges.push_within_capacity(Edge {
                index1,
                index2,
                distance : (samples[index1] - samples[index2]).squared_length(),
            });
        }
    }
    let mut edges = BinaryHeap::from(edges);

    let mut disjoint_set = DisjointSet::new(samples.len());
    while disjoint_set.connected_component_count() > cluster_count {
        let edge = edges.pop().unwrap();
        disjoint_set.merge(edge.index1, edge.index2);
    }
    disjoint_set.connceted_component_labels()
}

