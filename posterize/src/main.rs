#![feature(generic_nonzero)]

use posterize::Posterize;
use cluster::k_means::KMeanInit;

use image::io::Reader as ImageReader;
use image::DynamicImage;

use std::path::PathBuf;
use std::num::NonZero;

use anyhow::Result;

use clap::Parser;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Init {
    Llyod,
    KMeanPlusPlus,
}

impl From<Init> for KMeanInit {
    fn from(value: Init) -> Self {
        match value {
            Init::Llyod => Self::Llyod,
            Init::KMeanPlusPlus => Self::KMeanPlusPlus,
        }
    }
}

/// Posterize an image using k-mean-clustering algorithm.
#[derive(Parser)]
struct Cli {
    /// Input filepath
    input : PathBuf,
    /// Output filepath
    output : PathBuf,
    /// Number of color in output image/Parameter in k-mean-clustering algorithm
    k : NonZero<usize>,
    /// Initialization method in k-mean-clustering algorithm
    init : Init,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut image = ImageReader::open(cli.input)?.decode()?;
    match &mut image {
        DynamicImage::ImageLuma8(image) => image.posterize(cli.k, cli.init.into()),
        DynamicImage::ImageLumaA8(image) => image.posterize(cli.k, cli.init.into()),

        DynamicImage::ImageLuma16(image) => image.posterize(cli.k, cli.init.into()),
        DynamicImage::ImageLumaA16(image) => image.posterize(cli.k, cli.init.into()),

        DynamicImage::ImageRgb8(image)  => image.posterize(cli.k, cli.init.into()),
        DynamicImage::ImageRgba8(image) => image.posterize(cli.k, cli.init.into()),

        DynamicImage::ImageRgb16(image)  => image.posterize(cli.k, cli.init.into()),
        DynamicImage::ImageRgba16(image) => image.posterize(cli.k, cli.init.into()),

        DynamicImage::ImageRgb32F(image)  => image.posterize(cli.k, cli.init.into()),
        DynamicImage::ImageRgba32F(image) => image.posterize(cli.k, cli.init.into()),

        x => panic!("Unsupported image type {x:?}"),
    }
    image.save(cli.output)?;

    Ok(())
}

