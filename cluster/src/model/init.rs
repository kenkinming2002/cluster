use crate::math::Vector;
use crate::utils::slice_random_ext::SliceRandomExt;

use rand::prelude::*;

#[derive(Copy, Clone, Debug)]
pub enum ModelInit {
    Llyod,
    KMeanPlusPlus,
}

impl ModelInit {
    pub(crate) fn init<'a, R, const N: usize>(self, rng : &'a mut R, samples : &'a [Vector<N>], k : usize) -> Vec<Vector<N>>
    where
        R: Rng
    {
        match self {
            Self::Llyod => samples.choose_multiple(rng, k).copied().collect(),
            Self::KMeanPlusPlus => {
                let mut result = Vec::with_capacity(k);

                let mut iter = 0..k;
                let mut errors = vec![f64::INFINITY; samples.len()];

                // 1: Pick initial element and update weights. After that our weights array should
                //    be initialized and usable.
                if iter.next().is_some() {
                    let mean = samples.choose(rng).copied().unwrap();
                    errors.iter_mut().zip(samples.iter().copied()).for_each(|(error, sample)| *error = error.min((mean - sample).squared_length()));
                    result.push(mean);
                }

                // 2: Pick other elements and update weights.
                for _ in iter.by_ref() {
                    let Ok(mean) = samples.choose_with_weights(rng, &errors).copied() else { break };
                    errors.iter_mut().zip(samples.iter().copied()).for_each(|(error, sample)| *error = error.min((mean - sample).squared_length()));
                    result.push(mean);
                }

                // 3: It is possible that we break out of the previous loop early if our weights
                //    array is malformed in some way. For example, if there all samples share the
                //    same value, the weights array would be zeroed upon picking the first sample,
                //    which is obviously invalid (since the probability of picking each element
                //    would be 0/(0+0...+0)). In that case, we fall back to picking elemenet
                //    normally with replacement.
                for _ in iter.by_ref() {
                    let mean = samples.choose(rng).copied().unwrap();
                    errors.iter_mut().zip(samples.iter().copied()).for_each(|(error, sample)| *error = error.min((mean - sample).squared_length()));
                    result.push(mean);
                }

                result
            },
        }
    }
}
