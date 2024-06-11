use cluster::math::*;
use cluster::model::ClusterModel;
use cluster::model::init::ModelInit;
use cluster::model::k_means::*;
use cluster::model::gaussian_mixture::*;

use frei0r_rs::*;

use rand::prelude::*;
use itertools::Itertools;

use std::ffi::CString;

#[derive(PluginBase)]
pub struct PosterizePlugin {
    #[frei0r(explain = c"clustering model to use(choices: k-means, gaussian-mixture, default : k-means)")] model : CString,
    #[frei0r(explain = c"initialization algorithm to use for clustering(choices: llyod, k-means++, default: llyod)")] init : CString,
    #[frei0r(explain = c"number of clusters(default: 2)")] cluster_count : f64,
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
            model : CString::from(c"gaussian-mixture"),
            cluster_count : 2.0,
            init : CString::from(c"llyod"),
        }
    }

    fn update(&self, _time : f64, _width : usize, _height : usize, inframe : &[u32], outframe : &mut [u32]) {
        #[allow(clippy::redundant_guards)]
        let model = match self.model.as_c_str() {
            init if init == c"k-means" => ClusterModel::KMeans,
            init if init == c"gaussian-mixture" => ClusterModel::GaussianMixture,
            init => panic!("Unsupported initialization method {init}", init = init.to_string_lossy()),
        };

        #[allow(clippy::redundant_guards)]
        let init = match self.init.as_c_str() {
            init if init == c"llyod" => ModelInit::Llyod,
            init if init == c"k-means++" => ModelInit::KMeanPlusPlus,
            init => panic!("Unsupported initialization method {init}", init = init.to_string_lossy()),
        };

        let samples = inframe.iter().map(|pixel| Vector::from_array(pixel.to_le_bytes().map(|x| x as f64))).collect::<Vec<_>>();

        let sample_count = samples.len();
        let cluster_count = self.cluster_count as usize;
        match model {
            ClusterModel::KMeans => {
                let (cluster_means, sample_labels, _) = KMeans::new(sample_count, cluster_count).run(&samples, init, &mut thread_rng());
                for (sample_index, pixel) in outframe.iter_mut().enumerate() {
                    *pixel = u32::from_le_bytes(Vector::into_array(cluster_means[sample_labels[sample_index]]).map(|x| x as u8));
                }
            },
            ClusterModel::GaussianMixture => {
                let (_, cluster_means, _, _, _, _, posteriors) = GaussianMixture::new(sample_count, cluster_count).run(&samples, init, &mut thread_rng());
                for (sample_index, pixel) in outframe.iter_mut().enumerate() {
                    let label = (0..cluster_count).map(|cluster_index| posteriors[cluster_index * sample_count + sample_index]).position_max_by(f64::total_cmp).unwrap();
                    *pixel = u32::from_le_bytes(Vector::into_array(cluster_means[label]).map(|x| x as u8));
                }
            },
        }
    }

    fn update2(&self, _ : f64, _width : usize, _height : usize, _inframe1 : &[u32], _inframe2 : &[u32], _inframe3 : &[u32], _outframe : &mut [u32]) {
        unreachable!()
    }
}

plugin!(PosterizePlugin);
