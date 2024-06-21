use super::init::ClusterInit;

use math::prelude::*;
use rand::prelude::*;

/// Implementation of Gaussian mixture model Clustering algorithm.
#[derive(Debug, Copy, Clone)]
pub struct GaussianMixture<const N: usize> {
    pub sample_count : usize,
    pub cluster_count : usize,
}

impl<const N: usize> GaussianMixture<N> {
    /// Constructor.
    pub fn new(sample_count : usize, cluster_count : usize) -> Self {
        Self { sample_count, cluster_count, }
    }

    /// Gaussian mixture model initialization step.
    ///
    /// **Inputs**:  (sample_values) <br/>
    /// **Outputs**: (cluster_weights, cluster_means, cluster_covariances)
    pub fn init<R>(self, sample_values : &[Vector<N>], init : ClusterInit, rng : &mut R) -> (Vec<f64>, Vec<Vector<N>>, Vec<Matrix<N>>)
    where
        R: Rng
    {
        assert_eq!(self.sample_count, sample_values.len());

        let cluster_weights = vec![1.0 / self.cluster_count as f64; self.cluster_count];
        let cluster_means = init.init(rng, sample_values, self.cluster_count);
        let cluster_covariances = vec![Matrix::one() * 0.01; self.cluster_count];

        (cluster_weights, cluster_means, cluster_covariances)
    }

    /// Gaussian mixture model expectation step(Kinda).
    ///
    /// **Inputs**:  (sample_values, cluster_weights, cluster_means, cluster_covariances) <br/>
    /// **Outputs**: (priors, likelihoods, marginal_likelihoods, posteriors) i.e. All your Bayesian goodies
    pub fn e_step(self, sample_values : &[Vector<N>], cluster_weights : &[f64], cluster_means : &[Vector<N>], cluster_covariances : &[Matrix<N>]) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
        assert_eq!(self.sample_count, sample_values.len());
        assert_eq!(self.cluster_count, cluster_weights.len());
        assert_eq!(self.cluster_count, cluster_means.len());
        assert_eq!(self.cluster_count, cluster_covariances.len());

        // 1: Compute priors P(C) from current estimate of model parameters.
        let mut priors = vec![Default::default(); self.sample_count];
        for cluster_index in 0..self.cluster_count {
            priors[cluster_index] = cluster_weights[cluster_index];
        }

        // 2: Compute likelihoods P(X|C) from current estimate of model parameters.
        let mut likelihoods = vec![Default::default(); self.sample_count * self.cluster_count];
        for cluster_index in 0..self.cluster_count {
            let dist = MultivariateGaussian::new(cluster_means[cluster_index], cluster_covariances[cluster_index]);
            for sample_index in 0..self.sample_count {
                // Disallow zero likelihoods, sort of like add one smoothing I guess? This prevent
                // division by zero down the road. In particular, we do not want zero marginal
                // likelihood if a sample happen to be far away from all clusters.
                likelihoods[cluster_index * self.sample_count + sample_index] = dist.sample(sample_values[sample_index]).max(1e-16);
            }
        }

        // 3: Compute marginal likelihoods P(X) = sum(P(X|C_i)P(C_i)).
        let mut marginal_likelihoods = vec![Default::default(); self.sample_count];
        for sample_index in 0..self.sample_count {
            marginal_likelihoods[sample_index] = 0.0;
            for cluster_index in 0..self.cluster_count {
                marginal_likelihoods[sample_index] += likelihoods[cluster_index * self.sample_count + sample_index] * priors[cluster_index];
            }
        }

        // 4: Compute posteriors P(C|X) = P(X|C)*P(C)/P(X)
        let mut posteriors = vec![Default::default(); self.sample_count * self.cluster_count];
        for cluster_index in 0..self.cluster_count {
            for sample_index in 0..self.sample_count {
                posteriors[cluster_index * self.sample_count + sample_index] = likelihoods[cluster_index * self.sample_count + sample_index] * priors[cluster_index] / marginal_likelihoods[sample_index]; // <- Aforementioned potential Division By Zero
            }
        }

        (priors, likelihoods, marginal_likelihoods, posteriors)
    }

    /// Gaussian mixture model maximization step(Kinda).
    ///
    /// **Inputs**:  (sample_values, priors, likelihoods, marginal_likelihoods, posteriors) <br/>
    /// **Outputs**: (cluster_weights, cluster_means, cluster_covariances)
    pub fn m_step(self, sample_values : &[Vector<N>], priors : &[f64], likelihoods : &[f64], marginal_likelihoods : &[f64], posteriors : &[f64]) -> (Vec<f64>, Vec<Vector<N>>, Vec<Matrix<N>>) {
        assert_eq!(self.sample_count, sample_values.len());
        assert_eq!(self.sample_count, priors.len());
        assert_eq!(self.sample_count * self.cluster_count, likelihoods.len());
        assert_eq!(self.sample_count, marginal_likelihoods.len());
        assert_eq!(self.sample_count * self.cluster_count, posteriors.len());

        // 1: MLE estimate of cluster weights: weight(C) = sum(P(C|X)) / sample_count
        let mut cluster_weights = vec![Default::default(); self.cluster_count];
        for cluster_index in 0..self.cluster_count {
            let mut total = 0.0;
            for sample_index in 0..self.sample_count {
                total += posteriors[cluster_index * self.sample_count + sample_index];
            }
            cluster_weights[cluster_index] = total / self.sample_count as f64;
        }

        // 2: MLE estimate of cluster means: mean(C) = weighted_average(X, P(C|X))
        let mut cluster_means = vec![Default::default(); self.cluster_count];
        for cluster_index in 0..self.cluster_count {
            let mut total = Vector::zero();
            let mut weight = 0.0;
            for sample_index in 0..self.sample_count {
                total  += sample_values[sample_index] * posteriors[cluster_index * self.sample_count + sample_index];
                weight +=                               posteriors[cluster_index * self.sample_count + sample_index];
            }
            cluster_means[cluster_index] = total / weight;
        }

        // 3: MLE estimate of cluster covariances: covariance(C) = weighted_average((X-mean(C))(X-mean(C))^T, P(C|X)) * sample_count / (sample_count - 1) (With Bessel's correction)
        let mut cluster_covariances = vec![Default::default(); self.cluster_count];
        for cluster_index in 0..self.cluster_count {
            let mut total = Matrix::zero();
            let mut weight = 0.0;
            for sample_index in 0..self.sample_count {
                total  += (sample_values[sample_index] - cluster_means[cluster_index]).outer_product(sample_values[sample_index] - cluster_means[cluster_index]) * posteriors[cluster_index * self.sample_count + sample_index];
                weight +=                                                                                                                                          posteriors[cluster_index * self.sample_count + sample_index];
            }
            cluster_covariances[cluster_index] = total / weight * self.sample_count as f64 / (self.sample_count - 1) as f64;
        }

        (cluster_weights, cluster_means, cluster_covariances)
    }

    /// Gaussian mixture model algorithm.
    ///
    /// **Inputs**:  (sample_values) <br/>
    /// **Outputs**: (cluster_weights, cluster_means, cluster_covariances, priors, likelihoods, marginal_likelihoods, posteriors)
    pub fn run<R>(self, sample_values : &[Vector<N>], init : ClusterInit, rng : &mut R) -> (Vec<f64>, Vec<Vector<N>>, Vec<Matrix<N>>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)
    where
        R: Rng
    {
        let (cluster_weights, cluster_means, cluster_covariances) = self.init(sample_values, init, rng);

        let (mut priors, mut likelihoods, mut marginal_likelihoods, mut posteriors) = self.e_step(sample_values, &cluster_weights, &cluster_means, &cluster_covariances);
        let (mut cluster_weights, mut cluster_means, mut cluster_covariances)       = self.m_step(sample_values, &priors, &likelihoods, &marginal_likelihoods, &posteriors);
        loop {
            let (new_priors, new_likelihoods, new_marginal_likelihoods, new_posteriors) = self.e_step(sample_values, &cluster_weights, &cluster_means, &cluster_covariances);
            let terminate = false;
            priors = new_priors;
            likelihoods = new_likelihoods;
            marginal_likelihoods = new_marginal_likelihoods;
            posteriors = new_posteriors;
            if terminate {
                break
            }

            let (new_cluster_weights, new_cluster_means, new_cluster_covariances) = self.m_step(sample_values, &priors, &likelihoods, &marginal_likelihoods, &posteriors);
            let terminate = {
                let update_cluster_weights     = new_cluster_weights    .iter().copied().zip(cluster_weights    .iter().copied()).map(|(x, y)| x - y).mse();
                let update_cluster_means       = new_cluster_means      .iter().copied().zip(cluster_means      .iter().copied()).map(|(x, y)| x - y).mse();
                let update_cluster_covariances = new_cluster_covariances.iter().copied().zip(cluster_covariances.iter().copied()).map(|(x, y)| x - y).mse();
                update_cluster_weights + update_cluster_means + update_cluster_covariances <= 1e-4
            };
            cluster_weights = new_cluster_weights;
            cluster_means = new_cluster_means;
            cluster_covariances = new_cluster_covariances;
            if terminate {
                break
            }
        }

        (cluster_weights, cluster_means, cluster_covariances, priors, likelihoods, marginal_likelihoods, posteriors)
    }
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

        let sample_count = samples.len();
        let cluster_count = 2;
        let (cluster_weights, cluster_means, cluster_covariances, priors, likelihoods, marginal_likelihoods, posteriors) = GaussianMixture::new(sample_count, cluster_count).run(&samples, ClusterInit::KMeanPlusPlus, &mut thread_rng());

        assert!(cluster_weights    .iter().copied()                                            .all(f64::is_finite));
        assert!(cluster_means      .iter().copied().map(Vector::into_array).flatten()          .all(f64::is_finite));
        assert!(cluster_covariances.iter().copied().map(Matrix::into_array).flatten().flatten().all(f64::is_finite));

        assert!(likelihoods.         iter().copied().all(f64::is_finite));
        assert!(marginal_likelihoods.iter().copied().all(f64::is_finite));
        assert!(priors              .iter().copied().all(f64::is_finite));
        assert!(posteriors          .iter().copied().all(f64::is_finite));
    }
}

