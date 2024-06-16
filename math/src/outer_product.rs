use crate::prelude::*;

pub trait OuterProduct<Rhs> {
    type Output;
    fn outer_product(self, other : Rhs) -> Self::Output;
}

impl<const N: usize> OuterProduct<Vector<N>> for Vector<N> {
    type Output = Matrix<N>;
    fn outer_product(self, other : Vector<N>) -> Self::Output {
        let mut result = Matrix::zero();
        for j in 0..N {
            for i in 0..N {
                result[(j, i)] = self[j] * other[i];
            }
        }
        result
    }
}

