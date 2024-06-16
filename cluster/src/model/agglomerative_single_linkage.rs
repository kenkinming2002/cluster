use crate::utils::disjoint_set::DisjointSet;

use math::prelude::*;
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
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.total_cmp(&other.distance).reverse()
    }
}

/// Implementation of single-linkage agglomerative clustering.
pub fn agglomerative_single_linkage<const N: usize>(samples : &[Vector<N>], cluster_count : usize) -> Vec<usize> {
    // 1: Use Prim's algorithm to construct a MST where weights are distances between samples.
    let mut edges = Vec::new();
    {
        // We use the special value samples.len() in min_vertices array to indicate that a vertex
        // has been visited.
        let mut min_vertices = vec![0; samples.len()];
        let mut min_costs = samples.iter().map(|&sample| (sample - samples[0]).squared_length()).collect::<Vec<_>>();
        min_vertices[0] = samples.len();

        while let Some((vertex, (min_vertex, min_cost))) = std::iter::zip(min_vertices.iter().copied(), min_costs.iter().copied())
            .enumerate()
            .filter(|&(_, (min_vertex, _))| min_vertex != samples.len())
            .min_by(|&(_, (_, min_cost1)), &(_, (_, min_cost2))| f64::partial_cmp(&min_cost1, &min_cost2).unwrap())
        {
            edges.push(Edge { index1 : vertex, index2 : min_vertex, distance : min_cost });

            min_vertices[vertex] = samples.len();
            for other_vertex in 0..samples.len() {
                if min_vertices[other_vertex] != samples.len() {
                    let cost = (samples[other_vertex] - samples[vertex]).squared_length();
                    if min_costs[other_vertex] > cost {
                        min_costs[other_vertex] = cost;
                        min_vertices[other_vertex] = vertex;
                    }
                }
            }
        }
    }

    // 2: Construct a max heap
    let mut edges = BinaryHeap::from(edges);

    // 3: Use the heap
    let mut disjoint_set = DisjointSet::new(samples.len());
    while disjoint_set.connected_component_count() > cluster_count {
        let edge = edges.pop().unwrap();
        disjoint_set.merge(edge.index1, edge.index2);
    }
    disjoint_set.connceted_component_labels()
}

