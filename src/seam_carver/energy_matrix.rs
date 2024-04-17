use image::RgbImage;
use rayon::prelude::*;

pub struct EnergyMatrix {
    rows: Vec<Vec<u32>>,
}

impl EnergyMatrix {
    pub fn new(image: &RgbImage) -> Self {
        fn gradient(first: [u8; 3], second: [u8; 3]) -> u32 {
            let r = first[0].abs_diff(second[0]) as u32;
            let g = first[1].abs_diff(second[1]) as u32;
            let b = first[2].abs_diff(second[2]) as u32;

            r.pow(2) + g.pow(2) + b.pow(2)
        }

        let rows = (0..image.height())
            .into_par_iter()
            .map(|y| {
                let mut row = Vec::with_capacity(image.width() as usize);
                for x in 0..image.width() {
                    let left_neighbor = if x == 0 {
                        image.get_pixel(image.width() - 1, y)
                    } else {
                        image.get_pixel(x - 1, y)
                    };
                    let right_neighbor = if x == image.width() - 1 {
                        image.get_pixel(0, y)
                    } else {
                        image.get_pixel(x + 1, y)
                    };
                    let x_gradient = gradient(left_neighbor.0, right_neighbor.0);

                    let top_neighbor = if y == 0 {
                        image.get_pixel(x, image.height() - 1)
                    } else {
                        image.get_pixel(x, y - 1)
                    };
                    let bottom_neighbor = if y == image.height() - 1 {
                        image.get_pixel(x, 0)
                    } else {
                        image.get_pixel(x, y + 1)
                    };
                    let y_gradient = gradient(top_neighbor.0, bottom_neighbor.0);

                    row.push(x_gradient + y_gradient);
                }
                row
            })
            .collect();

        Self { rows }
    }

    pub fn get_energy(&self, x: usize, y: usize) -> u32 {
        self.rows[y][x]
    }

    pub fn width(&self) -> usize {
        self.rows[0].len()
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use image::open;

    use super::EnergyMatrix;

    #[test]
    fn test_em() {
        let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test_images", "test.jpeg"]
            .iter()
            .collect();
        let image = open(path).unwrap().to_rgb8();

        let em = EnergyMatrix::new(&image);
        assert_eq!(em.get_energy(0, 0), 146383);
        assert_eq!(em.get_energy(5, 5), 21999);
    }
}
