use crate::array_zip::ArrayZip;
use crate::permutation::Parity;
use crate::permutation::Permutations;

use std::ops::Index;
use std::ops::IndexMut;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

use std::ops::AddAssign;
use std::ops::SubAssign;
use std::ops::MulAssign;
use std::ops::DivAssign;

use std::iter::Sum;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const N: usize>([[f64; N]; N]);

impl<const N: usize> Default for Matrix<N> {
    fn default() -> Self {
        Self([(); N].map(|_| [(); N].map(|_| Default::default())))
    }
}

impl<const N: usize> Matrix<N> {
    pub fn from_fn<F>(mut cb : F) -> Self
    where
        F: FnMut((usize, usize)) -> f64
    {
        Self(std::array::from_fn(|j| std::array::from_fn(|i| cb((j, i)))))
    }

    pub fn from_array(values : [[f64; N]; N]) -> Self {
        Self(values)
    }

    pub fn into_array(self) -> [[f64; N]; N] {
        self.0
    }

    pub fn each_ref(&self) -> [[&f64; N]; N] {
        self.0.each_ref().map(|x| x.each_ref())
    }

    pub fn each_mut(&mut self) -> [[&mut f64; N]; N] {
        self.0.each_mut().map(|x| x.each_mut())
    }
}

impl<const N: usize> Index<(usize, usize)> for Matrix<N> {
    type Output = f64;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.each_ref()[index.0][index.1]
    }
}

impl<const N: usize> IndexMut<(usize, usize)> for Matrix<N> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.each_mut()[index.0][index.1]
    }
}

impl<const N: usize> Add<Matrix<N>> for Matrix<N> { type Output = Self; fn add(self, rhs: Self) -> Self::Output { Matrix(self.into_array().zip(rhs.into_array()).map(|(a, b)| a.zip(b).map(|(a, b)| a + b))) } }
impl<const N: usize> Sub<Matrix<N>> for Matrix<N> { type Output = Self; fn sub(self, rhs: Self) -> Self::Output { Matrix(self.into_array().zip(rhs.into_array()).map(|(a, b)| a.zip(b).map(|(a, b)| a - b))) } }
impl<const N: usize> Mul<Matrix<N>> for Matrix<N> { type Output = Self; fn mul(self, rhs: Self) -> Self::Output { Matrix(self.into_array().zip(rhs.into_array()).map(|(a, b)| a.zip(b).map(|(a, b)| a * b))) } }
impl<const N: usize> Div<Matrix<N>> for Matrix<N> { type Output = Self; fn div(self, rhs: Self) -> Self::Output { Matrix(self.into_array().zip(rhs.into_array()).map(|(a, b)| a.zip(b).map(|(a, b)| a / b))) } }

impl<const N: usize> Add<f64> for Matrix<N> { type Output = Self; fn add(self, rhs: f64) -> Self::Output { Matrix(self.into_array().map(|a| a.map(|a| a + rhs))) } }
impl<const N: usize> Sub<f64> for Matrix<N> { type Output = Self; fn sub(self, rhs: f64) -> Self::Output { Matrix(self.into_array().map(|a| a.map(|a| a - rhs))) } }
impl<const N: usize> Mul<f64> for Matrix<N> { type Output = Self; fn mul(self, rhs: f64) -> Self::Output { Matrix(self.into_array().map(|a| a.map(|a| a * rhs))) } }
impl<const N: usize> Div<f64> for Matrix<N> { type Output = Self; fn div(self, rhs: f64) -> Self::Output { Matrix(self.into_array().map(|a| a.map(|a| a / rhs))) } }

impl<const N: usize> AddAssign<Matrix<N>> for Matrix<N> { fn add_assign(&mut self, rhs: Self) { self.each_mut().zip(rhs.into_array()).map(|(a, b)| a.zip(b).map(|(a, b)| *a += b)); } }
impl<const N: usize> SubAssign<Matrix<N>> for Matrix<N> { fn sub_assign(&mut self, rhs: Self) { self.each_mut().zip(rhs.into_array()).map(|(a, b)| a.zip(b).map(|(a, b)| *a -= b)); } }
impl<const N: usize> MulAssign<Matrix<N>> for Matrix<N> { fn mul_assign(&mut self, rhs: Self) { self.each_mut().zip(rhs.into_array()).map(|(a, b)| a.zip(b).map(|(a, b)| *a *= b)); } }
impl<const N: usize> DivAssign<Matrix<N>> for Matrix<N> { fn div_assign(&mut self, rhs: Self) { self.each_mut().zip(rhs.into_array()).map(|(a, b)| a.zip(b).map(|(a, b)| *a /= b)); } }

impl<const N: usize> AddAssign<f64> for Matrix<N> { fn add_assign(&mut self, rhs: f64) { self.each_mut().map(|a| a.map(|a| *a += rhs)); } }
impl<const N: usize> SubAssign<f64> for Matrix<N> { fn sub_assign(&mut self, rhs: f64) { self.each_mut().map(|a| a.map(|a| *a -= rhs)); } }
impl<const N: usize> MulAssign<f64> for Matrix<N> { fn mul_assign(&mut self, rhs: f64) { self.each_mut().map(|a| a.map(|a| *a *= rhs)); } }
impl<const N: usize> DivAssign<f64> for Matrix<N> { fn div_assign(&mut self, rhs: f64) { self.each_mut().map(|a| a.map(|a| *a /= rhs)); } }

impl<const N: usize> Sum<Matrix<N>> for Matrix<N> {
    fn sum<I: Iterator<Item = Matrix<N>>>(iter: I) -> Self {
        iter.fold(Default::default(), |acc, x| acc + x)
    }
}

impl<const N: usize> Matrix<N> {
    pub fn zero() -> Self {
        Self::default()
    }

    pub fn one() -> Self {
        let mut result = Self::default();
        for i in 0..N {
            *result.each_mut()[i][i] = 1.0;
        }
        result
    }
}

impl<const N: usize> Matrix<N> {
    pub fn inverse(self) -> Self {
        let mut lhs = self;
        let mut rhs = Self::one();

        for pivot in 0..N {
            let row = (pivot..N).find(|&row| lhs[(row, pivot)] != 0.0).unwrap();
            if row != pivot {
                for i in 0..N {
                    unsafe {
                        std::ptr::swap(&mut lhs[(row, i)], &mut lhs[(pivot, i)]);
                        std::ptr::swap(&mut rhs[(row, i)], &mut rhs[(pivot, i)]);
                    }
                }
            }

            let c = lhs[(pivot, pivot)];
            lhs[(pivot, pivot)] = 1.0;

            for i in pivot+1..N { lhs[(pivot, i)] /= c; }
            for i in 0..N       { rhs[(pivot, i)] /= c; }

            for j in 0..N {
                if j != pivot {
                    let c = lhs[(j, pivot)];
                    lhs[(j, pivot)] = 0.0;

                    for i in pivot+1..N { lhs[(j, i)] -= c * lhs[(pivot, i)]; }
                    for i in 0..N       { rhs[(j, i)] -= c * rhs[(pivot, i)]; }
                }
            }
        }

        rhs
    }

    pub fn determinant(self) -> f64 {
        let mut result = 0.0;
        for (parity, permutation) in Permutations::<N>::new() {
            match parity {
                Parity::Even => result += permutation.into_iter().enumerate().map(|indices| self[indices]).product::<f64>(),
                Parity::Odd  => result -= permutation.into_iter().enumerate().map(|indices| self[indices]).product::<f64>(),
            }
        }
        result
    }
}

impl<const N: usize> std::fmt::Display for Matrix<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Improve the formatting
        let mut prefix = "[";
        for j in 0..N {
            write!(f, "{prefix}")?;
            write!(f, "[")?;

            let mut delim = "";
            for i in 0..N {
                write!(f, "{delim}")?;
                write!(f, "{value}", value = self[(j, i)])?;
                delim = " ";
            }

            write!(f, "]")?;
            prefix = "\n ";
        }
        writeln!(f, "]")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    use rand::prelude::*;

    fn random_matrix<const N: usize>() -> Matrix<N> {
        Matrix::<N>::from_fn(|_| thread_rng().gen_range(-1.0..1.0))
    }

    fn matrix_inverse_impl<const N: usize>() {
        let a = random_matrix::<N>();
        let b = a.inverse();

        let result = a.inner_product(b);
        let expected = Matrix::<N>::one();

        eprintln!("=====================");
        eprintln!("Matrix a:");
        eprintln!("=====================");
        eprintln!("{a}");
        eprintln!("");

        eprintln!("=====================");
        eprintln!("Matrix b:");
        eprintln!("=====================");
        eprintln!("{b}");
        eprintln!("");

        eprintln!("=====================");
        eprintln!("Matrix a * b");
        eprintln!("=====================");
        eprintln!("{result}");
        eprintln!("");

        for j in 0..N {
            for i in 0..N {
                assert!((result[(j, i)] - expected[(j, i)]) < 1e-5);
            }
        }
    }

    #[test] fn matrix_inverse_1() { matrix_inverse_impl::<1>(); }
    #[test] fn matrix_inverse_2() { matrix_inverse_impl::<2>(); }
    #[test] fn matrix_inverse_3() { matrix_inverse_impl::<3>(); }
    #[test] fn matrix_inverse_4() { matrix_inverse_impl::<4>(); }

    fn matrix_determinant_impl<const N: usize>() {
        let a = random_matrix::<N>();
        let b = a.inverse();

        let det_a = a.determinant();
        let det_b = b.determinant();

        let result = det_a * det_b;
        let expected = 1.0;

        eprintln!("=====================");
        eprintln!("Matrix a:");
        eprintln!("=====================");
        eprintln!("{a}");
        eprintln!("");

        eprintln!("=====================");
        eprintln!("Matrix b:");
        eprintln!("=====================");
        eprintln!("{b}");
        eprintln!("");

        eprintln!("=====================");
        eprintln!("det(a)          = {det_a}");
        eprintln!("det(b)          = {det_b}");
        eprintln!("det(a) * det(b) = {result}");
        eprintln!("");

        assert!((result - expected) < 1e-5)
    }

    #[test] fn matrix_determinant_1() { matrix_determinant_impl::<1>(); }
    #[test] fn matrix_determinant_2() { matrix_determinant_impl::<2>(); }
    #[test] fn matrix_determinant_3() { matrix_determinant_impl::<3>(); }
    #[test] fn matrix_determinant_4() { matrix_determinant_impl::<4>(); }
}
