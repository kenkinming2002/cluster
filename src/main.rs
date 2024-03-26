#![feature(generic_nonzero)]

use k_mean_clustering::KMeanClusteringState;
use k_mean_clustering::Vector;

use image::io::Reader as ImageReader;

use image::Pixel;
use image::Luma;
use image::LumaA;
use image::Rgb;
use image::Rgba;

use image::DynamicImage;
use image::ImageBuffer;

use rand::prelude::*;

use std::path::PathBuf;

use std::num::NonZero;

use clap::Parser;

trait MyPixel<const CHANNEL_COUNT : usize>: Copy {
    fn from_array(array : [f32; CHANNEL_COUNT]) -> Self;
    fn into_array(self) -> [f32; CHANNEL_COUNT];
}

impl MyPixel<1> for Luma<u8> {
    fn from_array(array : [f32; 1]) -> Self { Self(array.map(|x| x as u8)) }
    fn into_array(self) -> [f32; 1] { self.0.map(|x| x as f32) }
}

impl MyPixel<2> for LumaA<u8> {
    fn from_array(array : [f32; 2]) -> Self { Self(array.map(|x| x as u8)) }
    fn into_array(self) -> [f32; 2] { self.0.map(|x| x as f32) }
}

impl MyPixel<1> for Luma<u16> {
    fn from_array(array : [f32; 1]) -> Self { Self(array.map(|x| x as u16)) }
    fn into_array(self) -> [f32; 1] { self.0.map(|x| x as f32) }
}

impl MyPixel<2> for LumaA<u16> {
    fn from_array(array : [f32; 2]) -> Self { Self(array.map(|x| x as u16)) }
    fn into_array(self) -> [f32; 2] { self.0.map(|x| x as f32) }
}

impl MyPixel<3> for Rgb<u8> {
    fn from_array(array : [f32; 3]) -> Self { Self(array.map(|x| x as u8)) }
    fn into_array(self) -> [f32; 3] { self.0.map(|x| x as f32) }
}

impl MyPixel<4> for Rgba<u8> {
    fn from_array(array : [f32; 4]) -> Self { Self(array.map(|x| x as u8)) }
    fn into_array(self) -> [f32; 4] { self.0.map(|x| x as f32) }
}

impl MyPixel<3> for Rgb<u16> {
    fn from_array(array : [f32; 3]) -> Self { Self(array.map(|x| x as u16)) }
    fn into_array(self) -> [f32; 3] { self.0.map(|x| x as f32) }
}

impl MyPixel<4> for Rgba<u16> {
    fn from_array(array : [f32; 4]) -> Self { Self(array.map(|x| x as u16)) }
    fn into_array(self) -> [f32; 4] { self.0.map(|x| x as f32) }
}

impl MyPixel<3> for Rgb<f32> {
    fn from_array(array : [f32; 3]) -> Self { Self(array) }
    fn into_array(self) -> [f32; 3] { self.0 }
}

impl MyPixel<4> for Rgba<f32> {
    fn from_array(array : [f32; 4]) -> Self { Self(array) }
    fn into_array(self) -> [f32; 4] { self.0 }
}

trait MyImage<const CHANNEL_COUNT : usize> {
    type Pixel : MyPixel<CHANNEL_COUNT>;

    fn pixels(&self)         -> impl Iterator<Item = &Self::Pixel>;
    fn pixels_mut(&mut self) -> impl Iterator<Item = &mut Self::Pixel>;
}

impl<const CHANNEL_COUNT : usize, P, Container> MyImage<CHANNEL_COUNT> for ImageBuffer<P, Container>
where
    P: MyPixel<CHANNEL_COUNT>,
    P: Pixel,
    Container: std::ops::Deref<Target = [P::Subpixel]>,
    Container: std::ops::DerefMut<Target = [P::Subpixel]>,
{
    type Pixel = P;

    fn pixels(&self)         -> impl Iterator<Item = &Self::Pixel> { self.pixels() }
    fn pixels_mut(&mut self) -> impl Iterator<Item = &mut Self::Pixel> { self.pixels_mut() }
}

fn posterize<I, const CHANNEL_COUNT : usize>(image : &mut I, k : NonZero<usize>)
where
    I: MyImage<CHANNEL_COUNT>
{
    let mut rng = thread_rng();

    let positions = image.pixels().map(|pixel| Vector(pixel.into_array()));
    let means     = (0..k.get()).map(|_| Vector([(); CHANNEL_COUNT].map(|_| rng.gen_range(0.0..255.0))));

    let mut state = KMeanClusteringState::new(positions, means);
    while state.step() {}

    for (pixel, label) in std::iter::zip(image.pixels_mut(), state.labels()) {
        *pixel = I::Pixel::from_array(state.means().nth(label).unwrap().0);
    }
}

#[derive(Parser)]
struct Cli {
    input : PathBuf,
    output : PathBuf,
    k : NonZero<usize>,
}

fn main() {
    let cli = Cli::parse();

    let mut image = ImageReader::open(cli.input).unwrap().decode().unwrap();
    match &mut image {
        DynamicImage::ImageLuma8(image) => posterize(image, cli.k),
        DynamicImage::ImageLumaA8(image) => posterize(image, cli.k),

        DynamicImage::ImageLuma16(image) => posterize(image, cli.k),
        DynamicImage::ImageLumaA16(image) => posterize(image, cli.k),

        DynamicImage::ImageRgb8(image)  => posterize(image, cli.k),
        DynamicImage::ImageRgba8(image) => posterize(image, cli.k),

        DynamicImage::ImageRgb16(image)  => posterize(image, cli.k),
        DynamicImage::ImageRgba16(image) => posterize(image, cli.k),

        DynamicImage::ImageRgb32F(image)  => posterize(image, cli.k),
        DynamicImage::ImageRgba32F(image) => posterize(image, cli.k),

        x => panic!("Unsupported image type {x:?}"),
    }
    image.save(cli.output).unwrap();
}

