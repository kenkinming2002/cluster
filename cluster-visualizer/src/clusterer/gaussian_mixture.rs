use super::Render;
use super::Clusterer;
use super::pick_color;

use cluster::expectation_maximization::init::ClusterInit;
use cluster::expectation_maximization::gaussian_mixture::GaussianMixture;

use math::prelude::*;
use rand::prelude::*;

use itertools::Itertools;

fn lerp(a : f64, low : f64, high : f64) -> f64 {
    low + a * (high - low)
}

pub struct GaussianMixtureClusterer {
    gaussian_mixture : GaussianMixture<2>,

    sample_values : Vec<Vector<2>>,

    cluster_weights     : Vec<f64>,
    cluster_means       : Vec<Vector<2>>,
    cluster_covariances : Vec<Matrix<2>>,

    priors               : Vec<f64>,
    likelihoods          : Vec<f64>,
    marginal_likelihoods : Vec<f64>,
    posteriors           : Vec<f64>,
}

impl GaussianMixtureClusterer {
    pub fn new(samples : Vec<Vector<2>>, cluster_count : usize) -> Box<Self> {
        let gaussian_mixture = GaussianMixture::new(samples.len(), cluster_count);

        let sample_values = samples;
        let (cluster_weights, cluster_means, cluster_covariances) = gaussian_mixture.init(&sample_values, ClusterInit::KMeanPlusPlus, &mut thread_rng());
        let (priors, likelihoods, marginal_likelihoods, posteriors) = gaussian_mixture.e_step(&sample_values, &cluster_weights, &cluster_means, &cluster_covariances);

        Box::new(Self {
            gaussian_mixture,

            sample_values,

            cluster_weights,
            cluster_means,
            cluster_covariances,

            priors,
            likelihoods,
            marginal_likelihoods,
            posteriors,
        })
    }
}

impl Clusterer for GaussianMixtureClusterer {
    fn into_raw(self : Box<Self>) -> Vec<Vector<2>> {
        self.sample_values
    }

    fn update(&mut self) {
        let (cluster_weights, cluster_means, cluster_covariances) = self.gaussian_mixture.m_step(&self.sample_values, &self.priors, &self.likelihoods, &self.marginal_likelihoods, &self.posteriors);
        self.cluster_weights = cluster_weights;
        self.cluster_means = cluster_means;
        self.cluster_covariances = cluster_covariances;

        let (priors, likelihoods, marginal_likelihoods, posteriors) = self.gaussian_mixture.e_step(&self.sample_values, &self.cluster_weights, &self.cluster_means, &self.cluster_covariances);
        self.priors = priors;
        self.likelihoods = likelihoods;
        self.marginal_likelihoods = marginal_likelihoods;
        self.posteriors = posteriors;

    }

    fn render(&self, mut render : Render<'_>) {
        for (sample_index, sample_value) in self.sample_values.iter().enumerate() {
            let sample_label = (0..self.gaussian_mixture.cluster_count).map(|cluster_index| self.posteriors[cluster_index * self.gaussian_mixture.sample_count + sample_index]).position_max_by(f64::total_cmp).unwrap();
            let ratio = sample_label as f64 / self.gaussian_mixture.cluster_count as f64;
            let (r, g, b) = pick_color(ratio);
            render.draw_point(r, g, b, sample_value[0], sample_value[1], 5.0);
        }

        for (cluster_weight, cluster_mean, cluster_covariance) in itertools::izip!(&self.cluster_weights, &self.cluster_means, &self.cluster_covariances) {
            let size = lerp(*cluster_weight, 10.0, 30.0);
            render.draw_point(255, 255, 255, cluster_mean[0], cluster_mean[1], size);

            // Method from https://carstenschelp.github.io/2018/09/14/Plot_Confidence_Ellipse_001.html
            let p = cluster_covariance[(0, 1)] / (cluster_covariance[(0, 0)] * cluster_covariance[(1, 1)]).sqrt();
            let rx = (1.0 + p).sqrt();
            let ry = (1.0 - p).sqrt();
            let sx = 2.0 * cluster_covariance[(0, 0)].sqrt();
            let sy = 2.0 * cluster_covariance[(1, 1)].sqrt();
            let angle = std::f64::consts::FRAC_PI_2;
            render.draw_ellipse_scaled(255, 255, 255, cluster_mean[0], cluster_mean[1], rx, ry, angle, sx, sy);
        }
    }
}

