use cluster::math::Vector;
use cluster::model::init::ClusterInit;
use cluster::model::k_means::KMeans;

use rand::prelude::*;

use crate::render::Render;

use super::Clusterer;

fn lerp(a : f64, low : f64, high : f64) -> f64 {
    low + a * (high - low)
}

pub struct KMeansClusterer {
    k_means : KMeans<2>,

    sample_values : Vec<Vector<2>>,
    sample_labels : Vec<usize>,
    sample_errors : Vec<f64>,

    cluster_means : Vec<Vector<2>>,
}

impl KMeansClusterer {
    pub fn new(samples : Vec<Vector<2>>, cluster_count : usize) -> Box<Self> {
        let k_means = KMeans::new(samples.len(), cluster_count);

        let sample_values = samples;
        let (cluster_means,) = k_means.init(&sample_values, ClusterInit::KMeanPlusPlus, &mut thread_rng());
        let (sample_labels, sample_errors) = k_means.e_step(&sample_values, &cluster_means);

        Box::new(Self { k_means, cluster_means, sample_values, sample_labels, sample_errors, })
    }
}

impl Clusterer for KMeansClusterer {
    fn into_raw(self : Box<Self>) -> Vec<Vector<2>> {
        self.sample_values
    }

    fn update(&mut self) {
        let (cluster_means,) = self.k_means.m_step(&self.sample_values, &self.sample_labels, &self.sample_errors);
        self.cluster_means = cluster_means;

        let (sample_labels, sample_errors) = self.k_means.e_step(&self.sample_values, &self.cluster_means);
        self.sample_labels = sample_labels;
        self.sample_errors = sample_errors;
    }

    fn render(&self, mut render : Render<'_>) {
        for (sample_value, sample_label) in std::iter::zip(&self.sample_values, &self.sample_labels) {
            let r = lerp(*sample_label as f64 / self.k_means.cluster_count as f64, 32.0, 224.0) as u8;
            let g = lerp(*sample_label as f64 / self.k_means.cluster_count as f64, 224.0, 32.0) as u8;
            let b = lerp(*sample_label as f64 / self.k_means.cluster_count as f64, 64.0, 196.0) as u8;
            render.draw_point(r, g, b, sample_value[0], sample_value[1], 5.0);
        }

        for cluster_mean in &self.cluster_means {
            let r = 0;
            let g = 0;
            let b = 255;
            render.draw_point(r, g, b, cluster_mean[0], cluster_mean[1], 10.0);
        }
    }
}
