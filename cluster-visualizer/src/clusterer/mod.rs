pub mod none;
pub mod k_means;
pub mod gaussian_mixture;
pub mod affinity_propagation;
pub mod dbscan;

pub mod slink;
pub mod clink;
pub mod agglomerative;

pub use none::NoneClusterer;
pub use k_means::KMeansClusterer;
pub use gaussian_mixture::GaussianMixtureClusterer;
pub use affinity_propagation::AffinityPropagationClusterer;
pub use dbscan::DbscanClusterer;

pub use slink::SlinkClusterer;
pub use clink::ClinkClusterer;
pub use agglomerative::AgglomerativeClusterer;

use crate::render::Render;

use math::prelude::*;

pub trait Clusterer {
    /// Convert clusterer into its internal state.
    ///
    /// Currently, this only returns the original array samples used to construct the clusterer but
    /// you may imagine things like labels and cluster means being returned.
    fn into_raw(self : Box<Self>) -> Vec<Vector<2>>;

    fn update(&mut self);
    fn render(&self, render : Render<'_>);
}

pub fn random_samples() -> Vec<Vector<2>> {
    use rand::prelude::*;
    use rand_distr::Normal;

    const SAMPLE_CLUSTER_COUNT : usize = 10;
    const SAMPLE_CLUSTER_SIZE  : usize = 40;

    const SAMPLE_CLUSTER_VARIANCE_MIN : f64 = 0.01;
    const SAMPLE_CLUSTER_VARIANCE_MAX : f64 = 0.03;

    let mut rng = thread_rng();
    let mut samples = Vec::new();
    for _ in 0..SAMPLE_CLUSTER_COUNT {
        let mean_x = rng.gen_range(0.1..0.9);
        let mean_y = rng.gen_range(0.1..0.9);

        let var_x = rng.gen_range(SAMPLE_CLUSTER_VARIANCE_MIN..=SAMPLE_CLUSTER_VARIANCE_MAX);
        let var_y = rng.gen_range(SAMPLE_CLUSTER_VARIANCE_MIN..=SAMPLE_CLUSTER_VARIANCE_MAX);

        let dist_x = Normal::<f64>::new(mean_x, var_x).unwrap();
        let dist_y = Normal::<f64>::new(mean_y, var_y).unwrap();

        for _ in 0..SAMPLE_CLUSTER_SIZE {
            let x = dist_x.sample(&mut rng).clamp(0.05, 0.95);
            let y = dist_y.sample(&mut rng).clamp(0.05, 0.95);
            samples.push(Vector::from_array([x, y]));
        }
    }
    samples
}

pub enum ImagePlane {
    RG,
    GB,
    RB,
}

/// Load samples from a image.
///
/// Unfortunately, pixels usually composed of 3 components R, G, B, but we can only deal with two
/// of them.
pub fn image_samples(plane : ImagePlane) -> Vec<Vector<2>> {
    use crate::utils::choose_file;
    use image::io::Reader as ImageReader;

    let image_filepath = choose_file();
    let image = ImageReader::open(image_filepath).unwrap().decode().unwrap();
    image.to_rgb32f().pixels().map(|pixel| match plane {
        ImagePlane::RG => Vector::from_array([ pixel.0[0] as f64, pixel.0[1] as f64, ]),
        ImagePlane::RB => Vector::from_array([ pixel.0[0] as f64, pixel.0[2] as f64, ]),
        ImagePlane::GB => Vector::from_array([ pixel.0[1] as f64, pixel.0[2] as f64, ]),
    }).collect()
}

pub fn pick_color(value : f64) -> (u8, u8, u8) {
    let h = value * 6.0;
    let s = 1.0;
    let v = 1.0;

    let c = s * v;
    let x = c * (1.0 - (h.rem_euclid(2.0) - 1.0).abs());

    let (r1, g1, b1) = if h <= 1.0 {
        (c, x, 0.0)
    } else if h <= 2.0 {
        (x, c, 0.0)
    } else if h <= 3.0 {
        (0.0, c, x)
    } else if h <= 4.0 {
        (0.0, x, c)
    } else if h <= 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let m = v - c;
    let r = r1 + m;
    let g = g1 + m;
    let b = b1 + m;

    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}
