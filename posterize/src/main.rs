use posterize::Posterize;
use cluster::model::ClusterModel;
use cluster::model::init::ModelInit;

use image::io::Reader as ImageReader;
use image::DynamicImage;

use std::path::PathBuf;
use std::num::NonZero;

use anyhow::Result;

use clap::Parser;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OurClusterModel {
    KMeans,
    GaussianMixture,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OurModelInit {
    Llyod,
    KMeanPlusPlus,
}

impl From<OurClusterModel> for ClusterModel {
    fn from(value: OurClusterModel) -> Self {
        match value {
            OurClusterModel::KMeans => Self::KMeans,
            OurClusterModel::GaussianMixture => Self::GaussianMixture,
        }
    }
}

impl From<OurModelInit> for ModelInit {
    fn from(value: OurModelInit) -> Self {
        match value {
            OurModelInit::Llyod => Self::Llyod,
            OurModelInit::KMeanPlusPlus => Self::KMeanPlusPlus,
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
    /// Clustering algorithm to use
    model : OurClusterModel,
    /// Initialization method in clustering algorithm
    init : OurModelInit,
    /// Number of color in output image/Parameter in clustering algorithm
    k : NonZero<usize>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut image = ImageReader::open(cli.input)?.decode()?;
    match &mut image {
        DynamicImage::ImageLuma8(image) => image.posterize(cli.model.into(), cli.k, cli.init.into()),
        DynamicImage::ImageLumaA8(image) => image.posterize(cli.model.into(), cli.k, cli.init.into()),

        DynamicImage::ImageLuma16(image) => image.posterize(cli.model.into(), cli.k, cli.init.into()),
        DynamicImage::ImageLumaA16(image) => image.posterize(cli.model.into(), cli.k, cli.init.into()),

        DynamicImage::ImageRgb8(image)  => image.posterize(cli.model.into(), cli.k, cli.init.into()),
        DynamicImage::ImageRgba8(image) => image.posterize(cli.model.into(), cli.k, cli.init.into()),

        DynamicImage::ImageRgb16(image)  => image.posterize(cli.model.into(), cli.k, cli.init.into()),
        DynamicImage::ImageRgba16(image) => image.posterize(cli.model.into(), cli.k, cli.init.into()),

        DynamicImage::ImageRgb32F(image)  => image.posterize(cli.model.into(), cli.k, cli.init.into()),
        DynamicImage::ImageRgba32F(image) => image.posterize(cli.model.into(), cli.k, cli.init.into()),

        x => panic!("Unsupported image type {x:?}"),
    }
    image.save(cli.output)?;

    Ok(())
}

