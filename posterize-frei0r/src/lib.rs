use posterize::PosterizeMethod;
use posterize::ClusterInit;

use math::prelude::*;
use frei0r_rs::*;

use std::num::NonZero;
use std::ffi::CString;

#[derive(PluginBase)]
pub struct PosterizePlugin {
    #[frei0r(explain = c"clustering method to use(choices: k-means, gaussian-mixture, default : k-means)")] cluster_method : CString,
    #[frei0r(explain = c"initialization method to use for clustering(choices: llyod, k-means++, default: llyod)")] cluster_init : CString,
    #[frei0r(explain = c"number of clusters(default: 2)")] cluster_count : f64,
}

impl PosterizePlugin {
    fn posterize_method(&self) -> Option<PosterizeMethod> {
        if self.cluster_method.as_c_str() == c"llyod" {
            Some(PosterizeMethod::KMeans {
                cluster_init : self.init()?,
                cluster_count : self.cluster_count()?,
            })
        } else if self.cluster_method.as_c_str() == c"k-means" {
            Some(PosterizeMethod::GaussianMixture {
                cluster_init : self.init()?,
                cluster_count : self.cluster_count()?,
            })
        } else {
            None
        }
    }

    fn init(&self) -> Option<ClusterInit> {
        if self.cluster_init.as_c_str() == c"llyod" {
            Some(ClusterInit::Llyod)
        } else if self.cluster_init.as_c_str() == c"k-means" {
            Some(ClusterInit::KMeanPlusPlus)
        } else {
            None
        }
    }

    fn cluster_count(&self) -> Option<NonZero<usize>> {
        NonZero::new(self.cluster_count as usize)
    }

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
            cluster_method : CString::from(c"k-means"),
            cluster_count : 2.0,
            cluster_init : CString::from(c"k-means++"),
        }
    }

    fn update(&self, _time : f64, _width : usize, _height : usize, inframe : &[u32], outframe : &mut [u32]) {
        let mut samples = inframe
            .iter()
            .map(|pixel| pixel.to_le_bytes().map(|x| x as f64))
            .map(Vector::from_array)
            .collect::<Vec<_>>();

        let posterize_method = self.posterize_method().unwrap();
        posterize_method.posterize(&mut samples);

        let samples = samples
            .into_iter()
            .map(Vector::into_array)
            .map(|pixel| u32::from_le_bytes(pixel.map(|x| x as u8)));

        for (pixel, sample) in std::iter::zip(outframe, samples) {
            *pixel = sample;
        }
    }

    fn update2(&self, _ : f64, _width : usize, _height : usize, _inframe1 : &[u32], _inframe2 : &[u32], _inframe3 : &[u32], _outframe : &mut [u32]) {
        unreachable!()
    }
}

plugin!(PosterizePlugin);
