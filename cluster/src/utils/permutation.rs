#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Parity {
    Odd,
    Even,
}

impl Parity {
    pub fn flip(self) -> Self {
        match self {
            Self::Odd => Self::Even,
            Self::Even => Self::Odd,
        }
    }
}

pub struct Permutations<const N: usize>(PermutationsImpl<N>);

impl<const N: usize> Default for Permutations<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Permutations<N> {
    pub fn new() -> Self {
        Self(permutations_impl())
    }
}

impl<const N: usize> Iterator for Permutations<N> {
    type Item = (Parity, [usize; N]);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

type PermutationsImpl<const N: usize> = impl Iterator<Item = (Parity, [usize; N])>;

fn permutations_impl<const N: usize>() -> PermutationsImpl<N> {
    std::iter::from_coroutine(|| {
        let mut parity = Parity::Even;
        let mut item = std::array::from_fn(|i| i);
        loop {
            yield (parity, item);

            let Some(i) = (0..N-1).rev().find(|&i| item[i] < item[i+1]) else { break };
            let Some(j) = (i+1..N).rev().find(|&j| item[i] < item[j])   else { unreachable!() };

            item.swap(i, j);
            item[i+1..].reverse();
            if ((N - (i+1)) / 2 + 1) % 2 == 1 {
                parity = parity.flip();
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut permutations = Permutations::<3>::new();
        assert_eq!(permutations.next(), Some((Parity::Even, [0, 1, 2])));
        assert_eq!(permutations.next(), Some((Parity::Odd,  [0, 2, 1])));
        assert_eq!(permutations.next(), Some((Parity::Odd,  [1, 0, 2])));
        assert_eq!(permutations.next(), Some((Parity::Even, [1, 2, 0])));
        assert_eq!(permutations.next(), Some((Parity::Even, [2, 0, 1])));
        assert_eq!(permutations.next(), Some((Parity::Odd,  [2, 1, 0])));
        assert_eq!(permutations.next(), None);
    }
}
