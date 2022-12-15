use std::path::PathBuf;
use ulam::ToCoordinatesExt;

use clap::Parser;

#[derive(Parser)]
struct ImageArgs {
    #[arg(short, long)]
    matrix_dimension: usize,

    #[arg(short, long)]
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

    //floor[side_length / 2\ = cli.no_of_rows
    let center = cli.matrix_dimension as isize;
    ulam::UlamGenerator::default()
        .into_iter()
        .take(image_square_dimension.elements())
        .to_coordinates()
        .for_each(|(uv, (x, y))| {
            let row = center + x;
            let col = center + y;
            image_buffer.get_pixel_mut(col as u32, row as u32).0 = if uv.value == 1 {
                [0, 0, 0]
            } else if uv.is_prime {
                [171, 52, 175]
            } else {
                [52, 64, 175]
            }
        });

    Ok(image_buffer.save(cli.output_path)?)
}
