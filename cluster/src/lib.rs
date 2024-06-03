//! Implementation of various clustering algorithms.

/// Like [Iterator] but reusable.
pub trait Iterable {
    type Item;

    fn iter(&self) -> impl Iterator<Item = &Self::Item> + '_;
    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Self::Item> + '_;
}

/// Types that can be converted into [Iterable].
pub trait IntoIterable {
    type Item;
    type IntoIterable : Iterable<Item = Self::Item>;

    fn into_iterable(self) -> Self::IntoIterable;
}

/// A [Iterable] is of course [IntoIterable].
impl<I> IntoIterable for I
where
    I: Iterable
{
    type Item = <I as Iterable>::Item;
    type IntoIterable = Self;
    fn into_iterable(self) -> Self::IntoIterable {
        self
    }
}

impl<T> Iterable for Vec<T> {
    type Item = T;

    fn iter(&self) -> impl Iterator<Item = &Self::Item> + '_ { self.into_iter() }
    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Self::Item> + '_ { self.into_iter() }
}

pub trait Cluster<S> {
    fn cluster<I>(samples : I)
    where
        I: IntoIterable<Item = S>
    {
        let mut samples = samples.into_iterable();

        for sample in samples.iter() { }
        for sample in samples.iter_mut() { }
    }
}

pub fn foo<C, S>()
where
    C: Cluster<S>,
{
    let samples : Vec<S> = vec![];
    C::cluster(samples);
}
