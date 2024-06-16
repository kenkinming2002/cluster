use cluster::math::Vector;

use cluster::model::agglomerative_single_linkage::agglomerative_single_linkage;

use crate::render::Render;

use super::Clusterer;

fn lerp(a : f64, low : f64, high : f64) -> f64 {
    low + a * (high - low)
}

pub struct AgglomerativeSingleLinkageClusterer {
    cluster_count : usize,

    sample_values : Vec<Vector<2>>,
    sample_labels : Vec<usize>,
}

impl AgglomerativeSingleLinkageClusterer {
    pub fn new(samples : Vec<Vector<2>>, cluster_count : usize) -> Box<Self> {
        let sample_values = samples;
        let sample_labels = agglomerative_single_linkage(&sample_values, cluster_count);
        Box::new(Self { cluster_count, sample_values, sample_labels, })
    }
}

impl Clusterer for AgglomerativeSingleLinkageClusterer {
    fn into_raw(self : Box<Self>) -> Vec<Vector<2>> {
        self.sample_values
    }

    fn update(&mut self) {}

    fn render(&self, mut render : Render<'_>) {
        for (sample_value, sample_label) in std::iter::zip(&self.sample_values, &self.sample_labels) {
            let r = lerp(*sample_label as f64 / self.cluster_count as f64, 32.0, 224.0) as u8;
            let g = lerp(*sample_label as f64 / self.cluster_count as f64, 224.0, 32.0) as u8;
            let b = lerp(*sample_label as f64 / self.cluster_count as f64, 64.0, 196.0) as u8;
            render.draw_point(r, g, b, sample_value[0], sample_value[1], 5.0);
        }
    }
}

