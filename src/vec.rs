#[derive(Debug, Clone, Copy)]
pub struct Vector<const N: usize>(pub [f32; N]);

impl<const N: usize> Default for Vector<N> {
    fn default() -> Self {
        Self([0.0; N])
    }
}

impl<const N: usize> Vector<N> {
    pub fn length_squared(self) -> f32 {
        self.0.into_iter().map(|x| x*x).sum()
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }
}

impl<const N: usize> std::ops::Add for Vector<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self([0.0; N]);
        for i in 0..N {
            result.0[i] = self.0[i] + rhs.0[i];
        }
        result
    }
}

impl<const N: usize> std::ops::Sub for Vector<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Self([0.0; N]);
        for i in 0..N {
            result.0[i] = self.0[i] - rhs.0[i];
        }
        result
    }
}

impl<const N: usize> std::ops::Mul<f32> for Vector<N> {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut result = Self([0.0; N]);
        for i in 0..N {
            result.0[i] = self.0[i] * rhs;
        }
        result
    }
}

impl<const N: usize> std::ops::Div<f32> for Vector<N> {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        let mut result = Self([0.0; N]);
        for i in 0..N {
            result.0[i] = self.0[i] / rhs;
        }
        result
    }
}
