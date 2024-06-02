#![feature(generic_nonzero)]

use posterize::Convert;
use posterize::KMean;
use posterize::Vector;

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
        let mut rng = thread_rng();

        let values = inframe.iter().map(|pixel| Vector::from_array(pixel.to_le_bytes().map(Convert::convert))).collect::<Vec<_>>();
        let k = NonZero::new(self.k as usize).expect("k must be a non-zero positive integer");

        let kmean = KMean::new(values, k).init_llyod(&mut rng).run();
        let labels = kmean.labels();
        let means = kmean.means();

        for (pixel, label) in std::iter::zip(outframe, labels) {
            *pixel = u32::from_le_bytes(Vector::into_array(means[*label]).map(Convert::convert));
        }
    }

    fn update2(&self, _ : f64, _width : usize, _height : usize, _inframe1 : &[u32], _inframe2 : &[u32], _inframe3 : &[u32], _outframe : &mut [u32]) {
        unreachable!()
    }
}

plugin!(PosterizePlugin);
