use super::Render;
use super::Clusterer;
use super::pick_color;

use cluster::misc::dbscan::dbscan;

use math::prelude::*;

pub struct DbscanClusterer {
    samples : Vec<Vector<2>>,
    count : usize,
    labels : Vec<usize>,
}

impl DbscanClusterer {
    pub fn new(samples : Vec<Vector<2>>, epsilon : f64, min_pts : usize) -> Box<Self> {
        let (count, labels) = dbscan(&samples, epsilon, min_pts);
        Box::new(Self { samples, count, labels  })
    }
}

impl Clusterer for DbscanClusterer {
    fn into_raw(self : Box<Self>) -> Vec<Vector<2>> {
        self.samples
    }

    fn update(&mut self) {}

    fn render(&self, mut render : Render<'_>) {
        for (&sample, &label) in std::iter::zip(&self.samples, &self.labels) {
            if label != self.samples.len() {
                let ratio = label as f64 / self.count as f64;
                let (r, g, b) = pick_color(ratio);
                render.draw_point(r, g, b, sample[0], sample[1], 5.0);
            } else {
                render.draw_point(255, 0, 0, sample[0], sample[1], 10.0);
            }
        }
    }
}

