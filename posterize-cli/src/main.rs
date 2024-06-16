use posterize::PosterizeMethod;

use image::io::Reader as ImageReader;
use image::Rgb;

use math::prelude::*;

use anyhow::Result;
use anyhow::Context;

use clap::Parser;
use std::path::PathBuf;

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
    let     image = image.as_mut_rgb8().context("Only RGB image with 8-bit bit depth are supported")?;

    let mut samples = image
        .pixels()
        .map(|pixel| pixel.0.map(|subpixel| subpixel as f64))
        .map(Vector::from_array)
        .collect::<Vec<_>>();

    cli.method.posterize(&mut samples);

    let pixels = samples
        .into_iter()
        .map(Vector::into_array)
        .map(|pixel| Rgb(pixel.map(|subpixel| subpixel as u8)));

    image.pixels_mut().zip(pixels).for_each(|(lhs, rhs)| *lhs = rhs);
    image.save(cli.output)?;

    Ok(())
}

