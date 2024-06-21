use super::Render;
use super::Clusterer;
use super::pick_color;

use cluster::hierarchical::agglomerative::dendrogram::DendrogramSection;
use cluster::hierarchical::agglomerative::slink::slink;

use math::prelude::*;

pub struct SlinkClusterer {
    samples : Vec<Vector<2>>,
    cluster_count : usize,
    dendrogram_section : DendrogramSection,
}

impl SlinkClusterer {
    pub fn new(samples : Vec<Vector<2>>, cluster_count : usize) -> Box<Self> {
        let dendrogram = slink(&samples, |&sample1, &sample2| (sample1 - sample2).squared_length());
        let dendrogram_section = dendrogram.section_with_cluster_count(cluster_count);
        Box::new(Self { cluster_count, samples, dendrogram_section, })
    }
}

impl Clusterer for SlinkClusterer {
    fn into_raw(self : Box<Self>) -> Vec<Vector<2>> {
        self.samples
    }

    fn update(&mut self) {}

    fn render(&self, mut render : Render<'_>) {
        for (&sample, &label) in std::iter::zip(&self.samples, &self.dendrogram_section.labels) {
            let ratio = label as f64 / self.cluster_count as f64;
            let (r, g, b) = pick_color(ratio);
            render.draw_point(r, g, b, sample[0], sample[1], 5.0);
        }

        for &(index1, index2) in &self.dendrogram_section.edges {
            let sample1 = self.samples[index1];
            let sample2 = self.samples[index2];

            assert_eq!(self.dendrogram_section.labels[index1], self.dendrogram_section.labels[index2]);
            let label = self.dendrogram_section.labels[index1];

            let ratio = label as f64 / self.cluster_count as f64;
            let (r, g, b) = pick_color(ratio);
            render.draw_line(r, g, b, sample1[0], sample1[1], sample2[0], sample2[1]);
        }
    }
}


