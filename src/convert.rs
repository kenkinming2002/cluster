pub trait Convert<T> {
    fn convert(self) -> T;
}

macro_rules! impl_converts {
    (@base, ($from:ty), ($to:ty)) => {
        impl Convert<$to> for $from {
            fn convert(self) -> $to {
                self as $to
            }
        }
    };

    (@base, ($($froms:ty),*), ($to:ty, $($tos:ty),*)) => {
        impl_converts!(@base, ($($froms),*), ($($tos),*));
        impl_converts!(@base, ($($froms),*), ($to));
    };

    (@base, ($from:ty, $($froms:ty),*), ($($tos:ty),*)) => {
        impl_converts!(@base, ($from),       ($($tos),*));
        impl_converts!(@base, ($($froms),*), ($($tos),*));
    };

    ($($types:ty),*) => {
        impl_converts!(@base, ($($types),*), ($($types),*));
    };
}

impl_converts!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
