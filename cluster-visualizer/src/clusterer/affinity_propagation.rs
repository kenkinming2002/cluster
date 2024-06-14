use cluster::math::Vector;
use cluster::model::affinity_propagation::AffinityPropagation;

use crate::render::Render;

use super::Clusterer;

fn lerp(a : f64, low : f64, high : f64) -> f64 {
    low + a * (high - low)
}

pub struct AffinityPropagationClusterer {
    affinity_propagation : AffinityPropagation,
    damping : f64,

    sample_values : Vec<Vector<2>>,
    sample_labels : Vec<usize>,

    exemplers : Vec<usize>,
}

impl AffinityPropagationClusterer {
    pub fn new(samples : Vec<Vector<2>>, preference : f64, damping : f64) -> Box<Self> {
        let sample_values = samples;
        let affinity_propagation = AffinityPropagation::new(&sample_values, |&sample1, &sample2| -(sample1 - sample2).squared_length(), preference);
        let (exemplers, sample_labels) = affinity_propagation.exemplers_and_labels();

        Box::new(Self {
            affinity_propagation,
            damping,
            sample_values,
            sample_labels,
            exemplers,
        })
    }
}

impl Clusterer for AffinityPropagationClusterer {
    fn into_raw(self : Box<Self>) -> Vec<Vector<2>> {
        self.sample_values
    }

    fn update(&mut self) {
        self.affinity_propagation.update(self.damping);

        let (exemplers, sample_labels) = self.affinity_propagation.exemplers_and_labels();
        self.exemplers = exemplers;
        self.sample_labels = sample_labels;
    }

    fn render(&self, mut render : Render<'_>) {
        for (sample_value, sample_label) in std::iter::zip(&self.sample_values, &self.sample_labels) {
            let ratio = if !self.exemplers.is_empty() { *sample_label as f64 / self.exemplers.len() as f64 } else { 0.0 };
            let r = lerp(ratio, 32.0, 224.0) as u8;
            let g = lerp(ratio, 224.0, 32.0) as u8;
            let b = lerp(ratio, 64.0, 196.0) as u8;
            render.draw_point(r, g, b, sample_value[0], sample_value[1], 5.0);
        }
    }
}


