mod seam_carver;

use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use image::open;
use indicatif::{ProgressBar, ProgressStyle};
use seam_carver::{ResizeDimension, Seamcarver};

#[derive(Parser)]
struct Cli {
    image_path: PathBuf,
    #[arg(value_parser = clap::value_parser!(ResizeDimension))]
    dimensions: ResizeDimension,
    out_path: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let image = open(&cli.image_path)
        .context("could not open image")?
        .to_rgb8();
    let sc = Seamcarver::new(image, cli.dimensions)?;

    let bar = ProgressBar::new(sc.num_seams_to_be_carved() as u64)
        .with_style(ProgressStyle::with_template("{bar:70.cyan/blue} {pos:>7}/{len:7}").unwrap());
    let new_image = sc.carve(|| {
        bar.inc(1);
    });
    bar.finish();
    let out_path = cli.out_path.unwrap_or(cli.image_path);
    new_image.save(&out_path).context("could not save image")?;
    println!("Saved to {}", out_path.display());

    Ok(())
}
