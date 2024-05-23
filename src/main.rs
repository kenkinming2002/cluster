#![feature(generic_nonzero)]

use posterize::Posterize;

use image::io::Reader as ImageReader;
use image::DynamicImage;

use std::path::PathBuf;
use std::num::NonZero;

use anyhow::Result;

use clap::Parser;

/// Posterize an image using k-mean-clustering algorithm.
#[derive(Parser)]
struct Cli {
    /// Input filepath
    input : PathBuf,
    /// Output filepath
    output : PathBuf,
    /// Number of color in output image/Parameter in k-mean-clustering algorithm
    k : NonZero<usize>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut image = ImageReader::open(cli.input)?.decode()?;
    match &mut image {
        DynamicImage::ImageLuma8(image) => image.posterize(cli.k),
        DynamicImage::ImageLumaA8(image) => image.posterize(cli.k),

        DynamicImage::ImageLuma16(image) => image.posterize(cli.k),
        DynamicImage::ImageLumaA16(image) => image.posterize(cli.k),

        DynamicImage::ImageRgb8(image)  => image.posterize(cli.k),
        DynamicImage::ImageRgba8(image) => image.posterize(cli.k),

        DynamicImage::ImageRgb16(image)  => image.posterize(cli.k),
        DynamicImage::ImageRgba16(image) => image.posterize(cli.k),

        DynamicImage::ImageRgb32F(image)  => image.posterize(cli.k),
        DynamicImage::ImageRgba32F(image) => image.posterize(cli.k),

        x => panic!("Unsupported image type {x:?}"),
    }
    image.save(cli.output)?;

    Ok(())
}

