use super::Render;
use super::Clusterer;

use math::prelude::*;

pub struct NoneClusterer {
    samples : Vec<Vector<2>>,
}

impl NoneClusterer {
    pub fn new(samples : Vec<Vector<2>>) -> Box<Self> {
        Box::new(Self { samples })
    }
}

impl Clusterer for NoneClusterer {
    fn into_raw(self : Box<Self>) -> Vec<Vector<2>> {
        self.samples
    }

    fn update(&mut self) {}

    fn render(&self, mut render : Render<'_>) {
        for sample_value in &self.samples {
            let r = 255;
            let g = 0;
            let b = 0;
            render.draw_point(r, g, b, sample_value[0], sample_value[1], 5.0);
        }
    }
}
