use crate::array_zip::ArrayZip;
use crate::prelude::*;

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
pub struct Vector<const N: usize>([f64; N]);

impl<const N: usize> Default for Vector<N> {
    fn default() -> Self {
        Self([Default::default(); N])
    }
}

impl<const N: usize> Vector<N> {
    pub fn from_array(values : [f64; N]) -> Self {
        Self(values)
    }

    pub fn into_array(self) -> [f64; N] {
        self.0
    }

    pub fn each_ref(&self) -> [&f64; N] {
        self.0.each_ref()
    }

    pub fn each_mut(&mut self) -> [&mut f64; N] {
        self.0.each_mut()
    }
}

impl<const N: usize> Index<usize> for Vector<N> {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for Vector<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const N: usize> Add<Vector<N>> for Vector<N> { type Output = Self; fn add(self, rhs: Self) -> Self::Output { Vector(self.into_array().zip(rhs.into_array()).map(|(a, b)| a + b)) } }
impl<const N: usize> Sub<Vector<N>> for Vector<N> { type Output = Self; fn sub(self, rhs: Self) -> Self::Output { Vector(self.into_array().zip(rhs.into_array()).map(|(a, b)| a - b)) } }
impl<const N: usize> Mul<Vector<N>> for Vector<N> { type Output = Self; fn mul(self, rhs: Self) -> Self::Output { Vector(self.into_array().zip(rhs.into_array()).map(|(a, b)| a * b)) } }
impl<const N: usize> Div<Vector<N>> for Vector<N> { type Output = Self; fn div(self, rhs: Self) -> Self::Output { Vector(self.into_array().zip(rhs.into_array()).map(|(a, b)| a / b)) } }

impl<const N: usize> Add<f64> for Vector<N> { type Output = Self; fn add(self, rhs: f64) -> Self::Output { Vector(self.into_array().map(|a| a + rhs)) } }
impl<const N: usize> Sub<f64> for Vector<N> { type Output = Self; fn sub(self, rhs: f64) -> Self::Output { Vector(self.into_array().map(|a| a - rhs)) } }
impl<const N: usize> Mul<f64> for Vector<N> { type Output = Self; fn mul(self, rhs: f64) -> Self::Output { Vector(self.into_array().map(|a| a * rhs)) } }
impl<const N: usize> Div<f64> for Vector<N> { type Output = Self; fn div(self, rhs: f64) -> Self::Output { Vector(self.into_array().map(|a| a / rhs)) } }

impl<const N: usize> AddAssign<Vector<N>> for Vector<N> { fn add_assign(&mut self, rhs: Self) { self.0.each_mut().zip(rhs.into_array()).into_iter().for_each(|(a, b)| { *a += b }) } }
impl<const N: usize> SubAssign<Vector<N>> for Vector<N> { fn sub_assign(&mut self, rhs: Self) { self.0.each_mut().zip(rhs.into_array()).into_iter().for_each(|(a, b)| { *a -= b }) } }
impl<const N: usize> MulAssign<Vector<N>> for Vector<N> { fn mul_assign(&mut self, rhs: Self) { self.0.each_mut().zip(rhs.into_array()).into_iter().for_each(|(a, b)| { *a *= b }) } }
impl<const N: usize> DivAssign<Vector<N>> for Vector<N> { fn div_assign(&mut self, rhs: Self) { self.0.each_mut().zip(rhs.into_array()).into_iter().for_each(|(a, b)| { *a /= b }) } }

impl<const N: usize> AddAssign<f64> for Vector<N> { fn add_assign(&mut self, rhs: f64) { self.each_mut().into_iter().for_each(|a| { *a += rhs }) } }
impl<const N: usize> SubAssign<f64> for Vector<N> { fn sub_assign(&mut self, rhs: f64) { self.each_mut().into_iter().for_each(|a| { *a -= rhs }) } }
impl<const N: usize> MulAssign<f64> for Vector<N> { fn mul_assign(&mut self, rhs: f64) { self.each_mut().into_iter().for_each(|a| { *a *= rhs }) } }
impl<const N: usize> DivAssign<f64> for Vector<N> { fn div_assign(&mut self, rhs: f64) { self.each_mut().into_iter().for_each(|a| { *a /= rhs }) } }

impl<const N: usize> Sum<Vector<N>> for Vector<N> {
    fn sum<I: Iterator<Item = Vector<N>>>(iter: I) -> Self {
        iter.fold(Default::default(), |acc, x| acc + x)
    }
}

impl<const N: usize> Vector<N> {
    pub fn zero() -> Self {
        Self::default()
    }
}

impl<const N: usize> Vector<N> {
    pub fn squared_length(self) -> f64 {
        self.inner_product(self)
    }

    pub fn length(self) -> f64 {
        self.inner_product(self).sqrt()
    }
}

