use super::Render;
use super::Clusterer;

use cluster::hierarchical::agglomerative::naive::naive;
use cluster::hierarchical::agglomerative::naive::single_linkage;
use cluster::hierarchical::agglomerative::naive::complete_linkage;
use cluster::hierarchical::agglomerative::naive::average_linkage;

use math::prelude::*;

fn lerp(a : f64, low : f64, high : f64) -> f64 {
    low + a * (high - low)
}

pub struct AgglomerativeClusterer {
    samples : Vec<Vector<2>>,
    clusters : Vec<Vec<usize>>,
}

impl AgglomerativeClusterer {
    pub fn new_single_linkage(samples : Vec<Vector<2>>, cluster_count : usize) -> Box<Self> {
        let clusters = naive(samples.len(), cluster_count, single_linkage(|a, b| (samples[a] - samples[b]).squared_length()));
        Box::new(Self { samples, clusters, })
    }

    pub fn new_complete_linkage(samples : Vec<Vector<2>>, cluster_count : usize) -> Box<Self> {
        let clusters = naive(samples.len(), cluster_count, complete_linkage(|a, b| (samples[a] - samples[b]).squared_length()));
        Box::new(Self { samples, clusters, })
    }

    pub fn new_average_linkage(samples : Vec<Vector<2>>, cluster_count : usize) -> Box<Self> {
        let clusters = naive(samples.len(), cluster_count, average_linkage(|a, b| (samples[a] - samples[b]).length()));
        Box::new(Self { samples, clusters, })
    }
}

impl Clusterer for AgglomerativeClusterer {
    fn into_raw(self : Box<Self>) -> Vec<Vector<2>> {
        self.samples
    }

    fn update(&mut self) {}

    fn render(&self, mut render : Render<'_>) {
        let cluster_count = self.clusters.len();
        for (cluster_index, cluster) in self.clusters.iter().enumerate() {
            let ratio = cluster_index as f64 / cluster_count as f64;
            let r = lerp(ratio, 0.0, 256.0) as u8;
            let g = lerp(ratio, 256.0, 0.0) as u8;
            let b = lerp(ratio, 0.0, 256.0) as u8;
            for &sample_index in cluster {
                render.draw_point(r, g, b, self.samples[sample_index][0], self.samples[sample_index][1], 5.0);
            }
        }
    }
}
