#![feature(generic_nonzero)]

use k_mean_clustering::KMeanClusteringState;
use k_mean_clustering::Vector;

use image::io::Reader as ImageReader;

use image::DynamicImage;

use image::Rgb;
use image::RgbImage;

use image::Rgba;
use image::RgbaImage;

use rand::prelude::*;

use std::path::PathBuf;

use std::num::NonZero;

use clap::Parser;

trait MyPixel<const CHANNEL_COUNT : usize>: Copy {
    fn from_array(array : [u8; CHANNEL_COUNT]) -> Self;
    fn into_array(self) -> [u8; CHANNEL_COUNT];
}

trait MyImage<const CHANNEL_COUNT : usize> {
    type Pixel : MyPixel<CHANNEL_COUNT>;

    fn pixels(&self)         -> impl Iterator<Item = &Self::Pixel>;
    fn pixels_mut(&mut self) -> impl Iterator<Item = &mut Self::Pixel>;
}

impl MyPixel<3> for Rgb<u8> {
    fn from_array(array : [u8; 3]) -> Self { Self(array) }
    fn into_array(self) -> [u8; 3] { self.0 }
}

impl MyPixel<4> for Rgba<u8> {
    fn from_array(array : [u8; 4]) -> Self { Self(array) }
    fn into_array(self) -> [u8; 4] { self.0 }
}

impl MyImage<3> for RgbImage {
    type Pixel = Rgb<u8>;
    fn pixels(&self)         -> impl Iterator<Item = &Self::Pixel>     { self.pixels() }
    fn pixels_mut(&mut self) -> impl Iterator<Item = &mut Self::Pixel> { self.pixels_mut() }
}


impl MyImage<4> for RgbaImage {
    type Pixel = Rgba<u8>;
    fn pixels(&self)         -> impl Iterator<Item = &Self::Pixel>     { self.pixels() }
    fn pixels_mut(&mut self) -> impl Iterator<Item = &mut Self::Pixel> { self.pixels_mut() }
}

fn posterize<I, const CHANNEL_COUNT : usize>(image : &mut I, k : NonZero<usize>)
where
    I: MyImage<CHANNEL_COUNT>
{
    let mut rng = thread_rng();

    let positions = image.pixels().map(|pixel| Vector(pixel.into_array().map(|x| x as f32)));
    let means     = (0..k.get()).map(|_| Vector([(); CHANNEL_COUNT].map(|_| rng.gen_range(0.0..255.0))));

    let mut state = KMeanClusteringState::new(positions, means);
    while state.step() {}

    for (pixel, label) in std::iter::zip(image.pixels_mut(), state.labels()) {
        *pixel = I::Pixel::from_array(state.means().nth(label).unwrap().0.map(|x| x as u8));
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
        DynamicImage::ImageRgb8(image)  => posterize(image, cli.k),
        DynamicImage::ImageRgba8(image) => posterize(image, cli.k),
        _ => todo!(),
    }
    image.save(cli.output).unwrap();
}

