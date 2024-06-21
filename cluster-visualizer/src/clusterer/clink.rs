use super::Render;
use super::Clusterer;

use cluster::dendrogram::DendrogramSection;
use cluster::clink::clink;

use math::prelude::*;

fn lerp(a : f64, low : f64, high : f64) -> f64 {
    low + a * (high - low)
}

pub struct ClinkClusterer {
    samples : Vec<Vector<2>>,
    cluster_count : usize,
    dendrogram_section : DendrogramSection,
}

impl ClinkClusterer {
    pub fn new(samples : Vec<Vector<2>>, cluster_count : usize) -> Box<Self> {
        let dendrogram = clink(&samples, |&sample1, &sample2| (sample1 - sample2).squared_length());
        let dendrogram_section = dendrogram.section_with_cluster_count(cluster_count);
        Box::new(Self { cluster_count, samples, dendrogram_section, })
    }
}

impl Clusterer for ClinkClusterer {
    fn into_raw(self : Box<Self>) -> Vec<Vector<2>> {
        self.samples
    }

    fn update(&mut self) {}

    fn render(&self, mut render : Render<'_>) {
        for (&sample, &label) in std::iter::zip(&self.samples, &self.dendrogram_section.labels) {
            let ratio = label as f64 / self.cluster_count as f64;
            let g = lerp(ratio, 256.0, 0.0) as u8;
            let b = lerp(ratio, 0.0, 256.0) as u8;
            render.draw_point(0, g, b, sample[0], sample[1], 5.0);
        }

        for &(index1, index2) in &self.dendrogram_section.edges {
            let sample1 = self.samples[index1];
            let sample2 = self.samples[index2];

            assert_eq!(self.dendrogram_section.labels[index1], self.dendrogram_section.labels[index2]);
            let label = self.dendrogram_section.labels[index1];

            let ratio = label as f64 / self.cluster_count as f64;
            let g = lerp(ratio, 256.0, 0.0) as u8;
            let b = lerp(ratio, 0.0, 256.0) as u8;

            render.draw_line(255, g, b, sample1[0], sample1[1], sample2[0], sample2[1]);
        }
    }
}


