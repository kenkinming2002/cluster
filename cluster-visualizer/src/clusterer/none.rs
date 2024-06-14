use cluster::math::Vector;
use super::Clusterer;
use crate::render::Render;

pub struct NoneClusterer {
    sample_values : Vec<Vector<2>>,
}

impl Clusterer for NoneClusterer {
    fn from_sample_values(sample_values : Vec<Vector<2>>) -> Box<Self> where Self: Sized {
        Box::new(Self { sample_values })
    }

    fn into_sample_values(self : Box<Self>) -> Vec<Vector<2>> {
        self.sample_values
    }

    fn update(&mut self) {}

    fn render(&self, mut render : Render<'_>) {
        for sample_value in &self.sample_values {
            let r = 255;
            let g = 0;
            let b = 0;
            render.draw_point(r, g, b, sample_value[0], sample_value[1], 5.0);
        }
    }
}
