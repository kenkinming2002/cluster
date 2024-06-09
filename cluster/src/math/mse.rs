use super::Vector;
use super::Matrix;

trait Mse: Sized {
    fn mse<I>(iter: I) -> f64
       where I: Iterator<Item = Self>;
}

impl Mse for f64 {
    fn mse<I>(iter: I) -> f64
       where I: Iterator<Item = Self>
    {
        let mut count = 0usize;
        let total = iter.inspect(|_| count += 1).sum::<f64>();
        total / count as f64
    }
}

impl<const N: usize> Mse for Vector<N> {
    fn mse<I>(iter: I) -> f64
       where I: Iterator<Item = Self>
    {
        let mut count = 0usize;
        let total = iter.map(|x| x.into_array()).flatten().inspect(|_| count += 1).sum::<f64>();
        total / count as f64
    }
}

impl<const N: usize> Mse for Matrix<N> {
    fn mse<I>(iter: I) -> f64
       where I: Iterator<Item = Self>
    {
        let mut count = 0usize;
        let total = iter.map(|x| x.into_array()).flatten().flatten().inspect(|_| count += 1).sum::<f64>();
        total / count as f64
    }
}

pub trait MseIteratorExt {
    fn mse(self) -> f64;
}

impl<I> MseIteratorExt for I
where
    I: Iterator,
    I::Item: Mse,
{
    fn mse(self) -> f64 {
        Mse::mse(self)
    }
}
