use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

use std::ops::AddAssign;
use std::ops::SubAssign;
use std::ops::MulAssign;
use std::ops::DivAssign;

use std::iter::Sum;

trait ArrayZip<Rhs> {
    type Output;
    fn zip(self, rhs : Rhs) -> Self::Output;
}

impl<T, U, const N: usize> ArrayZip<[U; N]> for [T; N] {
    type Output = [(T, U); N];
    fn zip(self, rhs : [U; N]) -> Self::Output {
        let mut lhs = self.into_iter();
        let mut rhs = rhs.into_iter();
        std::array::from_fn(|_| (lhs.next().unwrap(), rhs.next().unwrap()))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector<T, const N: usize>([T; N]);

impl<T, const N: usize> Default for Vector<T, N>
where
    T: Default
{
    fn default() -> Self {
        Self([(); N].map(|_| Default::default()))
    }
}

impl<T, const N: usize> Vector<T, N> {
    pub fn from_array(values : [T; N]) -> Self {
        Self(values)
    }

    pub fn into_array(self) -> [T; N] {
        self.0
    }

    pub fn each_ref(&self) -> [&T; N] {
        self.0.each_ref()
    }

    pub fn each_mut(&mut self) -> [&mut T; N] {
        self.0.each_mut()
    }
}

impl<T, const N: usize> Vector<T, N> {
    pub fn dot(self, rhs : Vector<T, N>) -> T
    where
        T: Mul<Output = T>,
        T: Sum<T>,
    {
        let lhs = self.into_array().into_iter();
        let rhs = rhs.into_array().into_iter();
        std::iter::zip(lhs, rhs).map(|(a, b)| a * b).sum()
    }

    pub fn squared_length(self) -> T
    where
        T: Copy,
        T: Mul<Output = T>,
        T: Sum<T>,
    {
        self.dot(self)
    }
}

impl<T, const N: usize> Add<Vector<T, N>> for Vector<T, N> where T: Add<Output = T> { type Output = Self; fn add(self, rhs: Self) -> Self::Output { Vector(self.into_array().zip(rhs.into_array()).map(|(a, b)| a + b)) } }
impl<T, const N: usize> Sub<Vector<T, N>> for Vector<T, N> where T: Sub<Output = T> { type Output = Self; fn sub(self, rhs: Self) -> Self::Output { Vector(self.into_array().zip(rhs.into_array()).map(|(a, b)| a - b)) } }
impl<T, const N: usize> Mul<Vector<T, N>> for Vector<T, N> where T: Mul<Output = T> { type Output = Self; fn mul(self, rhs: Self) -> Self::Output { Vector(self.into_array().zip(rhs.into_array()).map(|(a, b)| a * b)) } }
impl<T, const N: usize> Div<Vector<T, N>> for Vector<T, N> where T: Div<Output = T> { type Output = Self; fn div(self, rhs: Self) -> Self::Output { Vector(self.into_array().zip(rhs.into_array()).map(|(a, b)| a / b)) } }

impl<T, const N: usize> Add<T> for Vector<T, N> where T: Add<Output = T>, T: Copy, { type Output = Self; fn add(self, rhs: T) -> Self::Output { Vector(self.into_array().map(|a| a + rhs)) } }
impl<T, const N: usize> Sub<T> for Vector<T, N> where T: Sub<Output = T>, T: Copy, { type Output = Self; fn sub(self, rhs: T) -> Self::Output { Vector(self.into_array().map(|a| a - rhs)) } }
impl<T, const N: usize> Mul<T> for Vector<T, N> where T: Mul<Output = T>, T: Copy, { type Output = Self; fn mul(self, rhs: T) -> Self::Output { Vector(self.into_array().map(|a| a * rhs)) } }
impl<T, const N: usize> Div<T> for Vector<T, N> where T: Div<Output = T>, T: Copy, { type Output = Self; fn div(self, rhs: T) -> Self::Output { Vector(self.into_array().map(|a| a / rhs)) } }

impl<T, const N: usize> AddAssign<Vector<T, N>> for Vector<T, N> where T: AddAssign { fn add_assign(&mut self, rhs: Self) { self.each_mut().zip(rhs.into_array()).into_iter().for_each(|(a, b)| { *a += b }) } }
impl<T, const N: usize> SubAssign<Vector<T, N>> for Vector<T, N> where T: SubAssign { fn sub_assign(&mut self, rhs: Self) { self.each_mut().zip(rhs.into_array()).into_iter().for_each(|(a, b)| { *a -= b }) } }
impl<T, const N: usize> MulAssign<Vector<T, N>> for Vector<T, N> where T: MulAssign { fn mul_assign(&mut self, rhs: Self) { self.each_mut().zip(rhs.into_array()).into_iter().for_each(|(a, b)| { *a *= b }) } }
impl<T, const N: usize> DivAssign<Vector<T, N>> for Vector<T, N> where T: DivAssign { fn div_assign(&mut self, rhs: Self) { self.each_mut().zip(rhs.into_array()).into_iter().for_each(|(a, b)| { *a /= b }) } }

impl<T, const N: usize> AddAssign<T> for Vector<T, N> where T: AddAssign, T: Copy, { fn add_assign(&mut self, rhs: T) { self.each_mut().into_iter().for_each(|a| { *a += rhs }) } }
impl<T, const N: usize> SubAssign<T> for Vector<T, N> where T: SubAssign, T: Copy, { fn sub_assign(&mut self, rhs: T) { self.each_mut().into_iter().for_each(|a| { *a -= rhs }) } }
impl<T, const N: usize> MulAssign<T> for Vector<T, N> where T: MulAssign, T: Copy, { fn mul_assign(&mut self, rhs: T) { self.each_mut().into_iter().for_each(|a| { *a *= rhs }) } }
impl<T, const N: usize> DivAssign<T> for Vector<T, N> where T: DivAssign, T: Copy, { fn div_assign(&mut self, rhs: T) { self.each_mut().into_iter().for_each(|a| { *a /= rhs }) } }

impl<T, const N: usize> Sum<Vector<T, N>> for Vector<T, N>
where
    T: Add<Output = T>,
    T: Default,
{
    fn sum<I: Iterator<Item = Vector<T, N>>>(iter: I) -> Self {
        let mut result = Default::default();
        for item in iter {
            result = result + item;
        }
        result
    }
}

