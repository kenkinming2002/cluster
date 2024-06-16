use super::Render;
use super::Clusterer;

use cluster::affinity_propagation::AffinityPropagation;

use math::prelude::*;


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
        let sample_labels = vec![0; sample_values.len()];

        let affinity_propagation = AffinityPropagation::new(&sample_values, |&sample1, &sample2| -(sample1 - sample2).squared_length(), preference);
        let exemplers = affinity_propagation.exemplers();

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
        self.exemplers = self.affinity_propagation.exemplers();
        if !self.exemplers.is_empty() {
            self.sample_labels = self.affinity_propagation.labels(&self.exemplers);
        }
    }

    fn render(&self, mut render : Render<'_>) {
        for (sample_index, (&sample_value, &sample_label)) in std::iter::zip(&self.sample_values, &self.sample_labels).enumerate() {
            // The sample_labels array we get are actually indices into the exemplers array which
            // then are indices into sample_values array. This ensure that labels array is kinda
            // contiguous.
            if let Some(&exempler_index) = self.exemplers.get(sample_label) {
                if sample_index == exempler_index {
                    // Exempler
                    render.draw_point(0, 0, 255, sample_value[0], sample_value[1], 10.0);
                } else {
                    // Other point
                    let ratio = if !self.exemplers.is_empty() { sample_label as f64 / self.exemplers.len() as f64 } else { 0.0 };
                    let r = lerp(ratio, 0.0, 255.0) as u8;
                    let g = lerp(ratio, 255.0, 0.0) as u8;
                    render.draw_line(r, g, 0, sample_value[0], sample_value[1], self.sample_values[exempler_index][0], self.sample_values[exempler_index][1]);
                    render.draw_point(r, g, 0, sample_value[0], sample_value[1], 5.0);
                }
            } else {
                // This mean we do not actually have any exemplers yet, and the labels are dummy
                // value we put in initially or some outdated values.
                render.draw_point(255, 0, 0, sample_value[0], sample_value[1], 5.0);
            }
        }
    }
}


