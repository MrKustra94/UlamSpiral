use std::path::PathBuf;
use ulam::ToCoordinatesExt;

use clap::Parser;

#[derive(Parser)]
struct ImageArgs {
    #[arg(short)]
    matrix_dimension: usize,

    #[arg(short)]
    output_path: PathBuf,
}

struct SquareMatrixDimensions(usize);

impl SquareMatrixDimensions {
    fn rows(&self) -> usize {
        self.0
    }

    fn columns(&self) -> usize {
        self.0
    }

    fn elements(&self) -> usize {
        self.0 * self.0
    }

    fn new(dimension: usize) -> Option<SquareMatrixDimensions> {
        if dimension & 1 == 1 {
            Some(SquareMatrixDimensions(dimension))
        } else {
            None
        }
    }
}
#[derive(Debug, thiserror::Error)]
#[error("Expected odd square matrix dimension. Got {current}.")]
struct InvalidDimensionError {
    current: usize,
}

const CENTER_COLOR: [u8; 3] = [0, 0, 0];
const PRIME_PURPLE_PIXEL_COLOR: [u8; 3] = [171, 52, 175];
const NON_PRIME_BLUE_PIXEL_COLOR: [u8; 3] = [52, 64, 175];

fn main() -> anyhow::Result<()> {
    let cli = ImageArgs::parse();

    let image_square_dimension =
        if let Some(dimension) = SquareMatrixDimensions::new(cli.matrix_dimension) {
            Ok(dimension)
        } else {
            Err(InvalidDimensionError {
                current: cli.matrix_dimension,
            })
        }?;
    let mut image_buffer = image::RgbImage::new(
        image_square_dimension.columns() as u32,
        image_square_dimension.rows() as u32,
    );

    let center = (cli.matrix_dimension/2) as isize;
    ulam::UlamGenerator::default()
        .into_iter()
        .take(image_square_dimension.elements() - 1)
        .to_coordinates()
        .for_each(|(uv, (x, y))| {
            let row = center + x;
            let col = center + y;
            image_buffer.get_pixel_mut(col as u32, row as u32).0 = if uv.value == 1 {
                CENTER_COLOR
            } else if uv.is_prime {
                PRIME_PURPLE_PIXEL_COLOR
            } else {
                NON_PRIME_BLUE_PIXEL_COLOR
            }
        });

    Ok(image_buffer.save(cli.output_path)?)
}
