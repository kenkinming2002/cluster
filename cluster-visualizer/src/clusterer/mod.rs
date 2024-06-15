pub mod none;
pub mod k_means;
pub mod gaussian_mixture;
pub mod agglomerative_single_linkage;
pub mod affinity_propagation;
pub mod dbscan;

pub use none::NoneClusterer;
pub use k_means::KMeansClusterer;
pub use gaussian_mixture::GaussianMixtureClusterer;
pub use agglomerative_single_linkage::AgglomerativeSingleLinkageClusterer;
pub use affinity_propagation::AffinityPropagationClusterer;
pub use dbscan::DbscanClusterer;

use cluster::math::Vector;

use crate::render::Render;

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
            let x = dist_x.sample(&mut rng).max(0.05).min(0.95);
            let y = dist_y.sample(&mut rng).max(0.05).min(0.95);
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
