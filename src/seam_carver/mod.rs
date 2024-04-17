mod cost_matrix;
mod energy_matrix;
mod resize_dimension;

use anyhow::bail;
use image::{
    imageops::{rotate270, rotate90},
    RgbImage,
};

use cost_matrix::CostMatrix;
use energy_matrix::EnergyMatrix;
pub use resize_dimension::ResizeDimension;

const MAX_GRADIENT: u32 = (u8::MAX as u32).pow(2) * 3 * 2;
pub const MAX_DIMENSION: u32 = u32::MAX / MAX_GRADIENT;

pub struct Seamcarver {
    image: RgbImage,
    carve_v_seams: u32,
    carve_h_seams: u32,
}

impl Seamcarver {
    pub fn new(image: RgbImage, rd: ResizeDimension) -> anyhow::Result<Self> {
        if image.width() > MAX_DIMENSION || image.height() > MAX_DIMENSION {
            bail!("image dimension too large");
        }

        let check_width = |new_width: u32| {
            if new_width > image.width() {
                bail!("width must be smaller than original width");
            }
            Ok(())
        };
        let check_height = |new_height: u32| {
            if new_height > image.height() {
                bail!("height must be smaller than original height");
            }
            Ok(())
        };
        let (new_width, new_height) = match rd {
            ResizeDimension::Width(w) => {
                check_width(w)?;
                (w, image.height())
            }
            ResizeDimension::Height(h) => {
                check_height(h)?;
                (image.width(), h)
            }
            ResizeDimension::WidthHeight(w, h) => {
                check_width(w)?;
                check_height(h)?;
                (w, h)
            }
        };
        let carve_v_seams = image.width() - new_width;
        let carve_h_seams = image.height() - new_height;

        Ok(Self {
            image,
            carve_v_seams,
            carve_h_seams,
        })
    }

    pub fn num_seams_to_be_carved(&self) -> u32 {
        self.carve_v_seams + self.carve_h_seams
    }

    fn carve_v_seam(&mut self) {
        let em = EnergyMatrix::new(&self.image);
        let cm = CostMatrix::new(&em);
        let min_seam = cm.min_cost_seam();

        let new_image_width = self.image.width() - 1;
        let mut new_image = RgbImage::new(self.image.width() - 1, self.image.height());
        for y in 0..self.image.height() {
            let col_to_remove = min_seam[y as usize] as u32;
            for x in 0..new_image_width {
                if x < col_to_remove {
                    new_image.put_pixel(x, y, self.image.get_pixel(x, y).clone());
                } else {
                    new_image.put_pixel(x, y, self.image.get_pixel(x + 1, y).clone());
                }
            }
        }
        self.image = new_image;
    }

    pub fn carve(mut self, on_seam_carved: impl Fn() -> ()) -> RgbImage {
        for _ in 0..self.carve_v_seams {
            self.carve_v_seam();
            on_seam_carved();
        }

        self.image = rotate90(&self.image);
        for _ in 0..self.carve_h_seams {
            self.carve_v_seam();
            on_seam_carved();
        }
        self.image = rotate270(&self.image);

        self.image
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use image::open;

    use super::*;

    #[test]
    fn test_carve() {
        let image_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test_images", "cows.jpg"]
            .iter()
            .collect();
        let image = open(image_path).unwrap().to_rgb8();
        let expected_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test_images", "expected.bmp"]
            .iter()
            .collect();
        let expected = open(expected_path).unwrap().to_rgb8();

        let sc = Seamcarver::new(image, ResizeDimension::WidthHeight(1000, 700)).unwrap();
        let new_image = sc.carve(|| {});

        assert_eq!(new_image, expected);
    }
}
