use crate::render::Render;

use cluster::math::Vector;
use cluster::math::Matrix;
use cluster::model::init::ModelInit;
use cluster::model::k_means::KMeans;
use cluster::model::gaussian_mixture::GaussianMixture;
use cluster::model::agglomerative::agglomerative_single_linkage;

use itertools::Itertools;

use rand::prelude::*;
use rand_distr::Normal;

const SAMPLE_CLUSTER_COUNT : usize = 10;
const SAMPLE_CLUSTER_SIZE  : usize = 40;

const SAMPLE_CLUSTER_VARIANCE_MIN : f64 = 0.01;
const SAMPLE_CLUSTER_VARIANCE_MAX : f64 = 0.03;

const SAMPLE_COUNT : usize = SAMPLE_CLUSTER_COUNT * SAMPLE_CLUSTER_SIZE;
const CLUSTER_COUNT : usize = 10;

fn lerp(a : f64, low : f64, high : f64) -> f64 {
    low + a * (high - low)
}

pub trait ClusterState {
    fn from_sample_values(sample_values : Vec<Vector<2>>) -> Self;
    fn into_sample_values(self) -> Vec<Vector<2>>;

    fn step(self) -> Self;

    fn render(&self, render : Render<'_>);
}

pub struct NoneClusterState {
    sample_values : Vec<Vector<2>>,
}

impl ClusterState for NoneClusterState {
    fn from_sample_values(sample_values : Vec<Vector<2>>) -> Self {
        Self { sample_values }
    }

    fn into_sample_values(self) -> Vec<Vector<2>> {
        self.sample_values
    }

    fn step(self) -> Self {
        self
    }

    fn render(&self, mut render : Render<'_>) {
        for sample_value in &self.sample_values {
            let r = 255;
            let g = 0;
            let b = 0;
            render.draw_point(r, g, b, sample_value[0], sample_value[1], 5.0);
        }
    }
}

pub struct KMeansClusterState {
    sample_values : Vec<Vector<2>>,
    sample_labels : Vec<usize>,
    sample_errors : Vec<f64>,

    cluster_means : Vec<Vector<2>>,
}

impl ClusterState for KMeansClusterState {
    fn from_sample_values(sample_values : Vec<Vector<2>>) -> Self {
        let kmean = KMeans::new(SAMPLE_COUNT, CLUSTER_COUNT);

        let (cluster_means,)               = kmean.init(&sample_values, ModelInit::KMeanPlusPlus, &mut thread_rng());
        let (sample_labels, sample_errors) = kmean.e_step(&sample_values, &cluster_means);

        Self {
            sample_values,
            sample_labels,
            sample_errors,

            cluster_means,
        }
    }

    fn into_sample_values(self) -> Vec<Vector<2>> {
        self.sample_values
    }

    fn step(self) -> Self {
        let kmean = KMeans::new(SAMPLE_COUNT, CLUSTER_COUNT);

        let sample_values = self.sample_values;
        let (cluster_means,)               = kmean.m_step(&sample_values, &self.sample_labels, &self.sample_errors);
        let (sample_labels, sample_errors) = kmean.e_step(&sample_values, &cluster_means);

        Self {
            sample_values,
            sample_labels,
            sample_errors,

            cluster_means,
        }
    }

    fn render(&self, mut render : Render<'_>) {
        for (sample_value, sample_label) in std::iter::zip(&self.sample_values, &self.sample_labels) {
            let r = lerp(*sample_label as f64 / CLUSTER_COUNT as f64, 32.0, 224.0) as u8;
            let g = lerp(*sample_label as f64 / CLUSTER_COUNT as f64, 224.0, 32.0) as u8;
            let b = lerp(*sample_label as f64 / CLUSTER_COUNT as f64, 64.0, 196.0) as u8;
            render.draw_point(r, g, b, sample_value[0], sample_value[1], 5.0);
        }

        for cluster_mean in &self.cluster_means {
            let r = 0;
            let g = 0;
            let b = 255;
            render.draw_point(r, g, b, cluster_mean[0], cluster_mean[1], 10.0);
        }
    }
}

pub struct GaussianMixtureClusterState {
    sample_values : Vec<Vector<2>>,

    cluster_weights     : Vec<f64>,
    cluster_means       : Vec<Vector<2>>,
    cluster_covariances : Vec<Matrix<2>>,

    priors               : Vec<f64>,
    likelihoods          : Vec<f64>,
    marginal_likelihoods : Vec<f64>,
    posteriors           : Vec<f64>,
}

impl ClusterState for GaussianMixtureClusterState {
    fn from_sample_values(sample_values : Vec<Vector<2>>) -> Self {
        let gaussian_mixture = GaussianMixture::new(SAMPLE_COUNT, CLUSTER_COUNT);

        let (cluster_weights, cluster_means, cluster_covariances)   = gaussian_mixture.init(&sample_values, ModelInit::KMeanPlusPlus, &mut thread_rng());
        let (priors, likelihoods, marginal_likelihoods, posteriors) = gaussian_mixture.e_step(&sample_values, &cluster_weights, &cluster_means, &cluster_covariances);

        Self {
            sample_values,

            cluster_weights,
            cluster_means,
            cluster_covariances,

            priors,
            likelihoods,
            marginal_likelihoods,
            posteriors,
        }
    }

    fn into_sample_values(self) -> Vec<Vector<2>> {
        self.sample_values
    }

    fn step(self) -> Self {
        let gaussian_mixture = GaussianMixture::new(SAMPLE_COUNT, CLUSTER_COUNT);

        let sample_values = self.sample_values;
        let (cluster_weights, cluster_means, cluster_covariances)   = gaussian_mixture.m_step(&sample_values, &self.priors, &self.likelihoods, &self.marginal_likelihoods, &self.posteriors);
        let (priors, likelihoods, marginal_likelihoods, posteriors) = gaussian_mixture.e_step(&sample_values, &cluster_weights, &cluster_means, &cluster_covariances);

        Self {
            sample_values,

            cluster_weights,
            cluster_means,
            cluster_covariances,

            priors,
            likelihoods,
            marginal_likelihoods,
            posteriors,
        }
    }

    fn render(&self, mut render : Render<'_>) {
        for (sample_index, sample_value) in self.sample_values.iter().enumerate() {
            let sample_label = (0..CLUSTER_COUNT).map(|cluster_index| self.posteriors[cluster_index * SAMPLE_COUNT + sample_index]).position_max_by(f64::total_cmp).unwrap();
            let r = lerp(sample_label as f64 / CLUSTER_COUNT as f64, 32.0, 224.0) as u8;
            let g = lerp(sample_label as f64 / CLUSTER_COUNT as f64, 224.0, 32.0) as u8;
            let b = lerp(sample_label as f64 / CLUSTER_COUNT as f64, 64.0, 196.0) as u8;
            render.draw_point(r, g, b, sample_value[0], sample_value[1], 5.0);
        }

        for (cluster_weight, cluster_mean, cluster_covariance) in itertools::izip!(&self.cluster_weights, &self.cluster_means, &self.cluster_covariances) {
            let r = 0;
            let g = 0;
            let b = 255;
            let size = lerp(*cluster_weight, 10.0, 30.0);
            render.draw_point(r, g, b, cluster_mean[0], cluster_mean[1], size);

            // Method from https://carstenschelp.github.io/2018/09/14/Plot_Confidence_Ellipse_001.html
            let p = cluster_covariance[(0, 1)] / (cluster_covariance[(0, 0)] * cluster_covariance[(1, 1)]).sqrt();
            let rx = (1.0 + p).sqrt();
            let ry = (1.0 - p).sqrt();
            let sx = 2.0 * cluster_covariance[(0, 0)].sqrt();
            let sy = 2.0 * cluster_covariance[(1, 1)].sqrt();
            let angle = std::f64::consts::FRAC_PI_2;
            render.draw_ellipse_scaled(r, g, b, cluster_mean[0], cluster_mean[1], rx, ry, angle, sx, sy);
        }
    }
}

pub struct AgglomerativeSingleLinkageClusterState {
    sample_values : Vec<Vector<2>>,
    sample_labels : Vec<usize>,
}

impl ClusterState for AgglomerativeSingleLinkageClusterState {
    fn from_sample_values(sample_values : Vec<Vector<2>>) -> Self {
        let sample_labels = agglomerative_single_linkage(&sample_values, CLUSTER_COUNT);
        Self { sample_values, sample_labels, }
    }

    fn into_sample_values(self) -> Vec<Vector<2>> {
        self.sample_values
    }

    fn step(self) -> Self {
        self
    }

    fn render(&self, mut render : Render<'_>) {
        for (sample_value, sample_label) in std::iter::zip(&self.sample_values, &self.sample_labels) {
            let r = lerp(*sample_label as f64 / CLUSTER_COUNT as f64, 32.0, 224.0) as u8;
            let g = lerp(*sample_label as f64 / CLUSTER_COUNT as f64, 224.0, 32.0) as u8;
            let b = lerp(*sample_label as f64 / CLUSTER_COUNT as f64, 64.0, 196.0) as u8;
            render.draw_point(r, g, b, sample_value[0], sample_value[1], 5.0);
        }
    }
}

pub enum ClusterStateAny {
    None(NoneClusterState),
    KMeans(KMeansClusterState),
    GaussianMixture(GaussianMixtureClusterState),
    AgglomerativeSingleLinkage(AgglomerativeSingleLinkageClusterState),
}

impl ClusterStateAny {
    pub fn reset() -> Self {
        let mut rng = thread_rng();
        let mut sample_values = Vec::new();
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
                sample_values.push(Vector::from_array([x, y]));
            }
        }

        Self::from_sample_values_none(sample_values)
    }

    pub fn from_sample_values_none(sample_values : Vec<Vector<2>>) -> Self {
        Self::None(NoneClusterState::from_sample_values(sample_values))
    }

    pub fn from_sample_values_k_means(sample_values : Vec<Vector<2>>) -> Self {
        Self::KMeans(KMeansClusterState::from_sample_values(sample_values))
    }

    pub fn from_sample_values_gaussian_mixture(sample_values : Vec<Vector<2>>) -> Self {
        Self::GaussianMixture(GaussianMixtureClusterState::from_sample_values(sample_values))
    }

    pub fn from_sample_values_agglomerative_single_linkage(sample_values : Vec<Vector<2>>) -> Self {
        Self::AgglomerativeSingleLinkage(AgglomerativeSingleLinkageClusterState::from_sample_values(sample_values))
    }

    pub fn into_sample_values(self) -> Vec<Vector<2>> {
        match self {
            Self::None(inner) => inner.into_sample_values(),
            Self::KMeans(inner) => inner.into_sample_values(),
            Self::GaussianMixture(inner) => inner.into_sample_values(),
            Self::AgglomerativeSingleLinkage(inner) => inner.into_sample_values(),
        }
    }

    pub fn step(self) -> Self {
        match self {
            Self::None(inner) => Self::None(inner.step()),
            Self::KMeans(inner) => Self::KMeans(inner.step()),
            Self::GaussianMixture(inner) => Self::GaussianMixture(inner.step()),
            Self::AgglomerativeSingleLinkage(inner) => Self::AgglomerativeSingleLinkage(inner.step()),
        }
    }

    pub fn render(&self, render : Render<'_>) {
        match self {
            Self::None(inner) => inner.render(render),
            Self::KMeans(inner) => inner.render(render),
            Self::GaussianMixture(inner) => inner.render(render),
            Self::AgglomerativeSingleLinkage(inner) => inner.render(render),
        }
    }
}
