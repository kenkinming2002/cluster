pub trait ArrayZip<Rhs> {
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


