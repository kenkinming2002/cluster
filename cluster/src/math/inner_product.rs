use super::Matrix;
use super::Vector;

pub trait InnerProduct<Rhs> {
    type Output;
    fn inner_product(self, other : Rhs) -> Self::Output;
}

impl<const N: usize> InnerProduct<Matrix<N>> for Matrix<N> {
    type Output = Matrix<N>;
    fn inner_product(self, other : Matrix<N>) -> Self::Output {
        let mut result = Self::Output::zero();
        for j in 0..N {
            for i in 0..N {
                for k in 0..N {
                    result[(j, i)] += self[(j, k)] * other[(k, i)]
                }
            }
        }
        result
    }
}

impl<const N: usize> InnerProduct<Vector<N>> for Matrix<N> {
    type Output = Vector<N>;
    fn inner_product(self, other : Vector<N>) -> Self::Output {
        let mut result = Self::Output::zero();
        for j in 0..N {
            for k in 0..N {
                result[j] += self[(j, k)] * other[k]
            }
        }
        result
    }
}

impl<const N: usize> InnerProduct<Vector<N>> for Vector<N> {
    type Output = f64;
    fn inner_product(self, other : Vector<N>) -> Self::Output {
        let mut result = 0.0;
        for k in 0..N {
            result += self[k] * other[k]
        }
        result
    }
}

