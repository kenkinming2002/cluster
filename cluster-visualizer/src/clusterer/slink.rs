use super::Render;
use super::Clusterer;

use cluster::slink::slink;

use math::prelude::*;

fn lerp(a : f64, low : f64, high : f64) -> f64 {
    low + a * (high - low)
}

pub struct SlinkClusterer {
    cluster_count : usize,

    samples : Vec<Vector<2>>,
    labels  : Vec<usize>,
}

impl SlinkClusterer {
    pub fn new(samples : Vec<Vector<2>>, cluster_count : usize) -> Box<Self> {
        let labels = slink(&samples, |&sample1, &sample2| (sample1 - sample2).squared_length()).with_cluster_count(cluster_count);
        Box::new(Self { cluster_count, samples, labels, })
    }
}

impl Clusterer for SlinkClusterer {
    fn into_raw(self : Box<Self>) -> Vec<Vector<2>> {
        self.samples
    }

    fn update(&mut self) {}

    fn render(&self, mut render : Render<'_>) {
        for (sample_value, sample_label) in std::iter::zip(&self.samples, &self.labels) {
            let r = lerp(*sample_label as f64 / self.cluster_count as f64, 32.0, 224.0) as u8;
            let g = lerp(*sample_label as f64 / self.cluster_count as f64, 224.0, 32.0) as u8;
            let b = lerp(*sample_label as f64 / self.cluster_count as f64, 64.0, 196.0) as u8;
            render.draw_point(r, g, b, sample_value[0], sample_value[1], 5.0);
        }
    }
}


