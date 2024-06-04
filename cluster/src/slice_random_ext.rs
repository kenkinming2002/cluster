#![allow(unused)]

use rand::prelude::*;
use rand::distributions::WeightedIndex;
use rand::distributions::WeightedError;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::uniform::SampleBorrow;

use std::ops::AddAssign;

pub trait SliceRandomExt {
    type Item;

    fn choose_with_weights<R, I, X>(&self, rng: &mut R, weights: I) -> Result<&Self::Item, WeightedError>
    where
        R: Rng + ?Sized,
        I: IntoIterator,
        I::Item: SampleBorrow<X>,
        X: SampleUniform + for<'a> AddAssign<&'a X> + PartialOrd + Clone + Default;

    fn choose_with_weights_mut<R, I, X>(&mut self, rng: &mut R, weights: I) -> Result<&mut Self::Item, WeightedError>
    where
        R: Rng + ?Sized,
        I: IntoIterator,
        I::Item: SampleBorrow<X>,
        X: SampleUniform + for<'a> AddAssign<&'a X> + PartialOrd + Clone + Default;
}

impl<T> SliceRandomExt for [T] {
    type Item = T;

    fn choose_with_weights<R, I, X>(&self, rng: &mut R, weights: I) -> Result<&Self::Item, WeightedError>
    where
        R: Rng + ?Sized,
        I: IntoIterator,
        I::Item: SampleBorrow<X>,
        X: SampleUniform + for<'a> AddAssign<&'a X> + PartialOrd + Clone + Default,
    {
        let distr = WeightedIndex::new(weights)?;
        Ok(&self[distr.sample(rng)])
    }

    fn choose_with_weights_mut<R, I, X>(&mut self, rng: &mut R, weights: I) -> Result<&mut Self::Item, WeightedError>
    where
        R: Rng + ?Sized,
        I: IntoIterator,
        I::Item: SampleBorrow<X>,
        X: SampleUniform + for<'a> AddAssign<&'a X> + PartialOrd + Clone + Default,
    {
        let distr = WeightedIndex::new(weights)?;
        Ok(&mut self[distr.sample(rng)])
    }
}
