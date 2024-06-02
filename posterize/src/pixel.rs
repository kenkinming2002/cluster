use image::Luma;
use image::LumaA;

use image::Rgb;
use image::Rgba;

pub trait Pixel: Copy {
    type Component;
    const COMPONENT_COUNT : usize;

    fn from_array(array : [Self::Component; Self::COMPONENT_COUNT]) -> Self;
    fn into_array(self) -> [Self::Component; Self::COMPONENT_COUNT];
}

macro_rules! impl_pixel {
    ($type:ident,$count:literal) => {
        impl<T: Copy> Pixel for $type<T> {
            type Component = T;
            const COMPONENT_COUNT : usize = $count;

            fn from_array(array : [Self::Component; Self::COMPONENT_COUNT]) -> Self { Self(array) }
            fn into_array(self) -> [Self::Component; Self::COMPONENT_COUNT] { self.0 }
        }
    }
}

impl_pixel!(Luma, 1);
impl_pixel!(LumaA, 2);
impl_pixel!(Rgb, 3);
impl_pixel!(Rgba, 4);

