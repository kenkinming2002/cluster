use itertools::Itertools;

fn lerp(a : f64, low : f64, high : f64) -> f64 {
    (1.0 - a) * low + a * high
}

pub struct AffinityPropagation {
    sample_count : usize,

    similarities : Vec<f64>,
    responsiblities : Vec<f64>,
    availabilities : Vec<f64>,
}

impl AffinityPropagation {
    pub fn new<T, S>(samples : &[T], similarity : S, preference : f64) -> Self
    where
        S: Fn(&T, &T) -> f64,
    {
        let sample_count = samples.len();

        let mut similarities = vec![0.0; sample_count * sample_count];
        for i in 0..samples.len() {
            for k in 0..samples.len() {
                similarities[i * samples.len() + k] = if i != k { similarity(&samples[i], &samples[k]) } else { preference };
            }
        }

        let responsiblities = vec![0.0; sample_count * sample_count];
        let availabilities  = vec![0.0; sample_count * sample_count];

        Self { sample_count, similarities, responsiblities, availabilities }
    }

    pub fn update(&mut self, damping : f64) {
        // Responsibilities update
        for i in 0..self.sample_count {
            for k in 0..self.sample_count {
                let mut new_responsibility = 0.0;

                new_responsibility += self.similarities[i * self.sample_count + k];
                new_responsibility -= (0..self.sample_count)
                    .filter(|&kp| kp != k)
                    .map(|kp| self.availabilities[i * self.sample_count + kp] + self.similarities[i * self.sample_count + kp])
                    .max_by(|a, b| f64::partial_cmp(a, b).unwrap())
                    .unwrap();

                self.responsiblities[i * self.sample_count + k] = lerp(damping, new_responsibility, self.responsiblities[i * self.sample_count + k]);
            }
        }

        // Availabilities update
        for i in 0..self.sample_count {
            for k in 0..self.sample_count {
                let mut new_availability = 0.0;

                for ip in 0..self.sample_count {
                    if ip != i && ip != k {
                        new_availability += self.responsiblities[ip * self.sample_count + k].max(0.0);
                    }
                }

                if i != k {
                    new_availability += self.responsiblities[k * self.sample_count + k];
                    new_availability = new_availability.min(0.0);
                }

                self.availabilities[i * self.sample_count + k] = lerp(damping, new_availability, self.availabilities[i * self.sample_count + k]);
            }
        }

    }

    pub fn exemplers(&self) -> Vec<usize> {
        (0..self.sample_count)
            .filter(|i| self.responsiblities[i * self.sample_count + i] + self.availabilities[i * self.sample_count + i] > 0.0)
            .collect()
    }

    pub fn labels(&self, exemplers : &[usize]) -> Vec<usize> {
        (0..self.sample_count)
            .map(|i| {
                exemplers
                    .iter()
                    .map(|k| self.responsiblities[i * self.sample_count + k])
                    .position_max_by(|a, b| f64::partial_cmp(a, b).unwrap())
                    .unwrap_or(0)
            })
            .collect()
    }
}

/// Implementation of affinity propagation clustering.
///
/// The number of output cluster is not fixed.
/// Return exemplers and labels.
pub fn affinity_propagation<T, S>(samples : &[T], similarity : S, preference : f64, damping : f64) -> (Vec<usize>, Vec<usize>)
where
    T: Copy,
    S: Fn(&T, &T) -> f64,
{
    let mut ap = AffinityPropagation::new(samples, similarity, preference);
    let mut exemplers = { ap.update(damping); ap.exemplers() };
    loop {
        let new_exemplers = { ap.update(damping); ap.exemplers() };
        if !exemplers.is_empty() && exemplers == new_exemplers {
            let labels = ap.labels(&exemplers);
            break (exemplers, labels)
        }
        exemplers = new_exemplers;
    }
}

