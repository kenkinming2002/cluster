#![feature(generic_nonzero)]

use k_mean_clustering::posterize;

use image::io::Reader as ImageReader;
use image::DynamicImage;

use std::path::PathBuf;
use std::num::NonZero;

use clap::Parser;

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

