use posterize::Posterize;
use posterize::PosterizeMethod;

use image::io::Reader as ImageReader;
use image::DynamicImage;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

/// Posterize an image using k-mean-clustering algorithm.
#[derive(Parser)]
struct Cli {
    /// Input filepath
    input : PathBuf,
    /// Output filepath
    output : PathBuf,

    /// Posterize method
    #[command(subcommand)]
    method : PosterizeMethod,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut image = ImageReader::open(cli.input)?.decode()?;
    match &mut image {
        DynamicImage::ImageLuma8(image) => image.posterize(cli.method),
        DynamicImage::ImageLumaA8(image) => image.posterize(cli.method),

        DynamicImage::ImageLuma16(image) => image.posterize(cli.method),
        DynamicImage::ImageLumaA16(image) => image.posterize(cli.method),

        DynamicImage::ImageRgb8(image)  => image.posterize(cli.method),
        DynamicImage::ImageRgba8(image) => image.posterize(cli.method),

        DynamicImage::ImageRgb16(image)  => image.posterize(cli.method),
        DynamicImage::ImageRgba16(image) => image.posterize(cli.method),

        DynamicImage::ImageRgb32F(image)  => image.posterize(cli.method),
        DynamicImage::ImageRgba32F(image) => image.posterize(cli.method),

        x => panic!("Unsupported image type {x:?}"),
    }
    image.save(cli.output)?;

    Ok(())
}

