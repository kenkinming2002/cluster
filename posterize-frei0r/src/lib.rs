#![feature(generic_nonzero)]

use cluster::vector::Vector;
use cluster::k_means::k_means_llyod;

use frei0r_rs::*;

use rand::prelude::*;
use std::num::NonZero;

#[derive(PluginBase)]
pub struct PosterizePlugin {
    #[frei0r(explain = c"number of clusters(default: 2)")] k : f64,
}

impl Plugin for PosterizePlugin {
    fn info() -> PluginInfo {
        PluginInfo {
            name : c"posterize",
            author : c"Ken Kwok",
            plugin_type : PluginType::Filter,
            color_model : ColorModel::RGBA8888,
            major_version : 1,
            minor_version : 0,
            explanation : c"image posterization effect using the k-mean clustering algorithm",
        }
    }

    fn new(_width : usize, _height : usize) -> Self {
        Self {
            k : 2.0,
        }
    }

    fn update(&self, _time : f64, _width : usize, _height : usize, inframe : &[u32], outframe : &mut [u32]) {
        let samples = inframe.iter().map(|pixel| Vector::from_array(pixel.to_le_bytes().map(|x| x as f32))).collect::<Vec<_>>();
        let k = NonZero::new(self.k as usize).expect("k must be a non-zero positive integer");
        let kmean = k_means_llyod(&mut thread_rng(), &samples, k);
        for (pixel, label) in std::iter::zip(outframe, kmean.labels.iter()) {
            *pixel = u32::from_le_bytes(Vector::into_array(kmean.means[*label]).map(|x| x as u8));
        }
    }

    fn update2(&self, _ : f64, _width : usize, _height : usize, _inframe1 : &[u32], _inframe2 : &[u32], _inframe3 : &[u32], _outframe : &mut [u32]) {
        unreachable!()
    }
}

plugin!(PosterizePlugin);
