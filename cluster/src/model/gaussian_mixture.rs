use crate::math::*;
use crate::model::init::ModelInit;

use rand::prelude::*;

use std::num::NonZero;

/// Result from training a Gaussian Mixture Model.
#[derive(Clone, Debug)]
pub struct GaussianMixtureResult<const N: usize>{
    pub sample_count : usize,
    pub samples      : Vec<Vector<N>>,

    pub cluster_count       : usize,
    pub cluster_weights     : Vec<f64>,
    pub cluster_means       : Vec<Vector<N>>,
    pub cluster_covariances : Vec<Matrix<N>>,

    /** P(X|C) */ pub likelihoods          : Vec<f64>,
    /** P(X)   */ pub marginal_likelihoods : Vec<f64>,

    /** P(C)   */ pub priors               : Vec<f64>,
    /** P(C|X) */ pub posteriors           : Vec<f64>,
}

pub struct GaussianMixture<'a, R, const N: usize>
where
    R: Rng
{
    rng : &'a mut R,
    init : ModelInit,

    sample_count : usize,
    samples      : Vec<Vector<N>>,

    cluster_count       : usize,

    cluster_weights     : Vec<f64>,
    cluster_means       : Vec<Vector<N>>,
    cluster_covariances : Vec<Matrix<N>>,

    likelihoods          : Vec<f64>,
    marginal_likelihoods : Vec<f64>,

    priors     : Vec<f64>,
    posteriors : Vec<f64>,

    delta : f64,
}


impl<'a, R, const N: usize> GaussianMixture<'a, R, N>
where
    R: Rng
{
    /// Constructor.
    ///
    /// The arguments are the same as in [TODO].
    #[allow(unused)]
    pub fn new<I>(rng : &'a mut R, init : ModelInit, k : NonZero<usize>, samples : I) -> Self
    where
        I: IntoIterator<Item = Vector<N>>
    {
        let samples = samples.into_iter().collect::<Vec<_>>();

        let sample_count = samples.len();
        let cluster_count = k.into();

        // This is unsafe, and probably invoke undefined behavior in plenty of different ways. But
        // a compiler that does not allow uninitialized POD is pretty stupid in my opinion.

        let cluster_weights     = unsafe { Box::new_uninit_slice(cluster_count).assume_init().into_vec() };
        let cluster_means       = unsafe { Box::new_uninit_slice(cluster_count).assume_init().into_vec() };
        let cluster_covariances = unsafe { Box::new_uninit_slice(cluster_count).assume_init().into_vec() };

        let likelihoods          = unsafe { Box::new_uninit_slice(sample_count * cluster_count).assume_init().into_vec() };
        let marginal_likelihoods = unsafe { Box::new_uninit_slice(sample_count)                .assume_init().into_vec() };

        let priors               = unsafe { Box::new_uninit_slice(cluster_count)               .assume_init().into_vec() };
        let posteriors           = unsafe { Box::new_uninit_slice(sample_count * cluster_count).assume_init().into_vec() };

        let error = 0.0;

        Self { rng, init, sample_count, samples, cluster_count, cluster_weights, cluster_means, cluster_covariances, likelihoods, marginal_likelihoods, priors, posteriors, delta: error }
    }

    pub fn init(&mut self) {
        self.cluster_weights.fill((self.cluster_count as f64).recip());
        self.cluster_means.copy_from_slice(&self.init.init(self.rng, &self.samples, self.cluster_count));
        self.cluster_covariances.fill(Matrix::one());
    }

    pub fn expectation_step(&mut self) {
        // 1: Compute priors P(X) from current estimate of model parameters.
        for cluster_index in 0..self.cluster_count {
            self.priors[cluster_index] = self.cluster_weights[cluster_index];
        }

        // 2: Compute likelihoods P(X|C) from current estimate of model parameters.
        for cluster_index in 0..self.cluster_count {
            let dist = MultivariateGaussian::new(self.cluster_means[cluster_index], self.cluster_covariances[cluster_index]);
            for sample_index in 0..self.sample_count {
                // Disallow zero likelihoods, sort of like add one smoothing I guess?
                // This prevent division by zero down the road.
                // In particular, we do not want zero marginal likelihood if a sample happen to be
                // far away from all clusters.
                self.likelihoods[cluster_index * self.sample_count + sample_index] = dist.sample(self.samples[sample_index]).max(1e-16);
            }
        }

        // 3: Compute marginal likelihoods P(X) = sum(P(X|C_i)P(C_i)).
        for sample_index in 0..self.sample_count {
            self.marginal_likelihoods[sample_index] = 0.0;
            for cluster_index in 0..self.cluster_count {
                self.marginal_likelihoods[sample_index] += self.likelihoods[cluster_index * self.sample_count + sample_index] * self.priors[cluster_index];
            }
        }

        // 4: Compute posteriors P(C|X) = P(X|C)*P(C)/P(X)
        for cluster_index in 0..self.cluster_count {
            for sample_index in 0..self.sample_count {
                self.posteriors[cluster_index * self.sample_count + sample_index] = self.likelihoods[cluster_index * self.sample_count + sample_index] * self.priors[cluster_index] / self.marginal_likelihoods[sample_index]; // <- Potential Division By Zero
            }
        }
    }

    pub fn maximization_step(&mut self) {
        self.delta = 0.0;

        // 1: MLE estimate of cluster weights: weight(C) = sum(P(C|X)) / sample_count
        for cluster_index in 0..self.cluster_count {
            let mut total = 0.0;
            for sample_index in 0..self.sample_count {
                total += self.posteriors[cluster_index * self.sample_count + sample_index];
            }

            let old = self.cluster_weights[cluster_index];
            let new = total / self.sample_count as f64;

            self.delta += (new - old) * (new - old);
            self.cluster_weights[cluster_index] = new;
        }

        // 2: MLE estimate of cluster means: mean(C) = weighted_average(X, P(C|X))
        for cluster_index in 0..self.cluster_count {
            let mut total = Vector::zero();
            let mut weight = 0.0;
            for sample_index in 0..self.sample_count {
                total  += self.samples[sample_index] * self.posteriors[cluster_index * self.sample_count + sample_index];
                weight +=                              self.posteriors[cluster_index * self.sample_count + sample_index];
            }

            let old = self.cluster_means[cluster_index];
            let new = total / weight;

            self.delta += (new - old).into_array().into_iter().map(|x| x * x).sum::<f64>();
            self.cluster_means[cluster_index] = new;
        }

        // 3: MLE estimate of cluster covariances: covariance(C) = weighted_average((X-mean(C))(X-mean(C))^T, P(C|X)) * sample_count / (sample_count - 1) (With Bessel's correction)
        for cluster_index in 0..self.cluster_count {
            let mut total = Matrix::zero();
            let mut weight = 0.0;
            for sample_index in 0..self.sample_count {
                total  += (self.samples[sample_index] - self.cluster_means[cluster_index]).outer_product(self.samples[sample_index] - self.cluster_means[cluster_index]) * self.posteriors[cluster_index * self.sample_count + sample_index];
                weight +=                                                                                                                                                  self.posteriors[cluster_index * self.sample_count + sample_index];
            }

            let old = self.cluster_covariances[cluster_index];
            let new = total / weight * self.sample_count as f64 / (self.sample_count - 1) as f64;

            self.delta += (new - old).into_array().into_iter().flatten().map(|x| x * x).sum::<f64>();
            self.cluster_covariances[cluster_index] = total / weight * self.sample_count as f64 / (self.sample_count - 1) as f64;
        }

        self.delta /= self.cluster_count as f64;
    }

    pub fn finish(self) -> GaussianMixtureResult<N> {
        GaussianMixtureResult {
            sample_count : self.sample_count,
            samples : self.samples,
            cluster_count : self.cluster_count,
            cluster_weights : self.cluster_weights,
            cluster_means : self.cluster_means,
            cluster_covariances : self.cluster_covariances,
            likelihoods : self.likelihoods,
            marginal_likelihoods : self.marginal_likelihoods,
            priors : self.priors,
            posteriors : self.posteriors,
        }
    }
}

/// Implementation of Gaussian Mixture Model
pub fn gaussian_mixture<R, const N: usize, I>(rng : &mut R, init : ModelInit, k : NonZero<usize>, samples : I) -> GaussianMixtureResult<N>
where
    R: Rng,
    I: IntoIterator<Item = Vector<N>>
{
    let mut state = GaussianMixture::new(rng, init, k, samples);
    state.init();
    for k in 1.. {
        state.expectation_step();
        state.maximization_step();

        eprintln!("cluster: gaussian mixture model: iteration {k} => delta = {delta}", delta = state.delta);
        if state.delta <= 1e-3 {
            break
        }
    }
    state.expectation_step();
    state.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stable() {
        let samples = [
            Vector::from_array([11.0]),
            Vector::from_array([11.5]),
            Vector::from_array([12.0]),
            Vector::from_array([12.5]),
            Vector::from_array([13.0]),
            Vector::from_array([13.5]),

            Vector::from_array([81.0]),
            Vector::from_array([81.5]),
            Vector::from_array([82.0]),
            Vector::from_array([82.5]),
            Vector::from_array([83.0]),
            Vector::from_array([83.5]),
        ];

        let result = gaussian_mixture(&mut thread_rng(), ModelInit::KMeanPlusPlus, NonZero::new(2).unwrap(), samples);

        assert!(result.cluster_weights    .iter().copied()                                            .all(f64::is_finite));
        assert!(result.cluster_means      .iter().copied().map(Vector::into_array).flatten()          .all(f64::is_finite));
        assert!(result.cluster_covariances.iter().copied().map(Matrix::into_array).flatten().flatten().all(f64::is_finite));

        assert!(result.likelihoods.         iter().copied().all(f64::is_finite));
        assert!(result.marginal_likelihoods.iter().copied().all(f64::is_finite));
        assert!(result.priors              .iter().copied().all(f64::is_finite));
        assert!(result.posteriors          .iter().copied().all(f64::is_finite));
    }
}
