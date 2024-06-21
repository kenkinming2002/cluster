use super::init::ClusterInit;

use math::prelude::Vector;
use rand::prelude::*;

/// Implementation of K-Means Clustering algorithm.
#[derive(Debug, Copy, Clone)]
pub struct KMeans<const N: usize> {
    pub sample_count : usize,
    pub cluster_count : usize,
}

impl<const N: usize> KMeans<N> {
    /// Constructor.
    pub fn new(sample_count : usize, cluster_count : usize) -> Self {
        Self { sample_count, cluster_count, }
    }

    /// K-Means initialization step.
    ///
    /// **Inputs**:  (sample_values) <br/>
    /// **Outputs**: (cluster_means)
    pub fn init<R>(self, sample_values : &[Vector<N>], init : ClusterInit, rng : &mut R) -> (Vec<Vector<N>>, )
    where
        R: Rng
    {
        assert_eq!(self.sample_count, sample_values.len());
        (init.init(rng, sample_values, self.cluster_count), )
    }

    /// K-Means expectation step.
    ///
    /// **Inputs**:  (sample_values, cluster_means) <br/>
    /// **Outputs**: (sample_labels, sample_errors)
    pub fn e_step(self, sample_values : &[Vector<N>], cluster_means : &[Vector<N>]) -> (Vec<usize>, Vec<f64>) {
        assert_eq!(self.sample_count, sample_values.len());
        assert_eq!(self.cluster_count, cluster_means.len());

        let mut sample_labels = vec![self.cluster_count; self.sample_count];
        let mut sample_errors = vec![f64::INFINITY;      self.sample_count];
        for sample_index in 0..self.sample_count {
            for cluster_index in 0..self.cluster_count {
                let error = (sample_values[sample_index] - cluster_means[cluster_index]).squared_length();
                if sample_errors[sample_index] > error {
                    sample_labels[sample_index] = cluster_index;
                    sample_errors[sample_index] = error;
                }
            }
        }
        (sample_labels, sample_errors)
    }

    /// K-Means maximization step.
    ///
    /// **Inputs**:  (sample_values, sample_labels, sample_errors) <br/>
    /// **Outputs**: (cluster_means)
    pub fn m_step(self, sample_values : &[Vector<N>], sample_labels : &[usize], sample_errors : &[f64]) -> (Vec<Vector<N>>,) {
        assert_eq!(self.sample_count, sample_values.len());
        assert_eq!(self.sample_count, sample_labels.len());
        assert_eq!(self.sample_count, sample_errors.len());

        let mut cluster_totals = vec![Vector::zero(); self.cluster_count];
        let mut cluster_counts = vec![0usize;         self.cluster_count];
        for sample_index in 0..self.sample_count {
            cluster_totals[sample_labels[sample_index]] += sample_values[sample_index];
            cluster_counts[sample_labels[sample_index]] += 1;
        }

        let mut cluster_means = vec![Vector::zero(); self.cluster_count];
        for cluster_index in 0..self.cluster_count {
            cluster_means[cluster_index] = if cluster_counts[cluster_index] != 0 {
                cluster_totals[cluster_index] / cluster_counts[cluster_index] as f64
            } else {
                // TODO: Allow caller to specify what to do in this case.
                Vector::zero()
            }
        }

        (cluster_means,)
    }

    /// K-Means algorithm.
    ///
    /// Current termination condition is if sample_labels stop changing. An alternative is to
    /// detect if cluster_means stop changing below some threshold.
    ///
    /// **Inputs**:  (sample_values) <br/>
    /// **Outputs**: (cluster_means, sample_labels, sample_errors)
    pub fn run<R>(self, sample_values : &[Vector<N>], init : ClusterInit, rng : &mut R) -> (Vec<Vector<N>>, Vec<usize>, Vec<f64>)
    where
        R: Rng
    {
        let (cluster_means,) = self.init(sample_values, init, rng);

        let (mut sample_labels, mut sample_errors) = self.e_step(sample_values, &cluster_means);
        let (mut cluster_means,)                   = self.m_step(sample_values, &sample_labels, &sample_errors);
        loop {
            let (new_sample_labels, new_sample_errors) = self.e_step(sample_values, &cluster_means);
            let terminate = new_sample_labels.iter().eq(sample_labels.iter());
            sample_labels = new_sample_labels;
            sample_errors = new_sample_errors;
            if terminate {
                break (cluster_means, sample_labels, sample_errors)
            }

            let (new_cluster_means,) = self.m_step(sample_values, &sample_labels, &sample_errors);
            let terminate = false;
            cluster_means = new_cluster_means;
            if terminate {
                break (cluster_means, sample_labels, sample_errors)
            }
        }
    }
}

