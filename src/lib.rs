#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct GenMetadata {
    direction: Direction,
    extend_on_end: bool,
    max_line_length: usize,
    index: usize,
    primes: Vec<usize>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UlamValue {
    pub value: usize,
    pub is_prime: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct UlamGenerator {
    next_value: UlamValue,
    next_metadata: GenMetadata,
}

impl Default for UlamGenerator {
    fn default() -> Self {
        UlamGenerator {
            next_value: UlamValue {
                value: 1,
                is_prime: false,
            },
            next_metadata: GenMetadata {
                direction: Direction::Right,
                extend_on_end: false,
                max_line_length: 1,
                index: 1,
                primes: vec![],
            },
        }
    }
}

impl Iterator for UlamGenerator {
    type Item = (UlamValue, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        let return_value = (self.next_value, self.next_metadata.direction);

        let next_number = self.next_value.value + 1;
        let is_prime = {
            let current_primes = &self.next_metadata.primes;
            current_primes.is_empty() || current_primes.iter().all(|x| next_number % x != 0)
        };
        if is_prime {
            self.next_metadata.primes.push(next_number);
        }
        self.next_value.value = next_number;
        self.next_value.is_prime = is_prime;

        let current_index = &self.next_metadata.index;
        let current_line_length = &self.next_metadata.max_line_length;
        if current_index == current_line_length {
            self.next_metadata.direction = self.next_metadata.direction.turn_left();
            if self.next_metadata.extend_on_end {
                self.next_metadata.max_line_length = current_line_length + 1;
            }
            self.next_metadata.extend_on_end = !self.next_metadata.extend_on_end;
            self.next_metadata.index = 1;
        } else {
            self.next_metadata.index = current_index + 1;
        }

        Some(return_value)
    }
}

pub struct ToCoordinates<I> {
    iter: I,
    coordinates: (isize, isize),
}

impl<I> ToCoordinates<I> {
    fn center(iter: I) -> ToCoordinates<I> {
        ToCoordinates {
            iter,
            coordinates: (0, 0),
        }
    }
}

pub trait ToCoordinatesExt: Sized + Iterator {
    fn to_coordinates(self) -> ToCoordinates<Self>;
}

impl<I: Iterator> ToCoordinatesExt for I {
    fn to_coordinates(self) -> ToCoordinates<Self> {
        ToCoordinates::center(self)
    }
}

impl<I> Iterator for ToCoordinates<I>
where
    I: Iterator<Item = (UlamValue, Direction)>,
{
    type Item = (UlamValue, (isize, isize));

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| {
            let current_coordinates @ (row, col) = self.coordinates;
            let next_coordinates = match x.1 {
                Direction::Up => (row - 1, col),
                Direction::Down => (row + 1, col),
                Direction::Left => (row, col - 1),
                Direction::Right => (row, col + 1),
            };
            self.coordinates = next_coordinates;
            (x.0, current_coordinates)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Direction, GenMetadata, ToCoordinatesExt, UlamGenerator, UlamValue};

    #[test]
    fn direction_turn_left_four_times_should_round_trip() {
        assert_eq!(
            Direction::Left,
            Direction::Up.turn_left(),
            "From up to left."
        );
        assert_eq!(
            Direction::Down,
            Direction::Left.turn_left(),
            "From left to down."
        );
        assert_eq!(
            Direction::Right,
            Direction::Down.turn_left(),
            "From down to right."
        );
        assert_eq!(
            Direction::Up,
            Direction::Right.turn_left(),
            "From right to up."
        )
    }

    #[test]
    fn ulam_generator_default_value_should_point_to_first_node() {
        let default = UlamGenerator::default();
        let expected = UlamGenerator {
            next_value: UlamValue {
                value: 1,
                is_prime: false,
            },
            next_metadata: GenMetadata {
                direction: Direction::Right,
                extend_on_end: false,
                max_line_length: 1,
                index: 1,
                primes: vec![],
            },
        };
        assert_eq!(expected, default)
    }

    #[test]
    fn ulam_generator_should_generate_correct_spiral_of_two() {
        let generated = UlamGenerator::default()
            .into_iter()
            .take(2)
            .collect::<Vec<(UlamValue, Direction)>>();

        let expected = vec![
            (
                UlamValue {
                    value: 1,
                    is_prime: false,
                },
                Direction::Right,
            ),
            (
                UlamValue {
                    value: 2,
                    is_prime: true,
                },
                Direction::Up,
            ),
        ];
        assert_eq!(expected, generated);
    }

    #[test]
    fn ulam_generator_should_generate_correct_spiral_of_four() {
        let generated = UlamGenerator::default()
            .into_iter()
            .take(4)
            .collect::<Vec<(UlamValue, Direction)>>();

        let expected = vec![
            (
                UlamValue {
                    value: 1,
                    is_prime: false,
                },
                Direction::Right,
            ),
            (
                UlamValue {
                    value: 2,
                    is_prime: true,
                },
                Direction::Up,
            ),
            (
                UlamValue {
                    value: 3,
                    is_prime: true,
                },
                Direction::Left,
            ),
            (
                UlamValue {
                    value: 4,
                    is_prime: false,
                },
                Direction::Left,
            ),
        ];
        assert_eq!(expected, generated);
    }

    #[test]
    fn ulam_generator_should_generate_correct_spiral_of_ten() {
        let generated = UlamGenerator::default()
            .into_iter()
            .take(10)
            .collect::<Vec<(UlamValue, Direction)>>();

        let expected = vec![
            (
                UlamValue {
                    value: 1,
                    is_prime: false,
                },
                Direction::Right,
            ),
            (
                UlamValue {
                    value: 2,
                    is_prime: true,
                },
                Direction::Up,
            ),
            (
                UlamValue {
                    value: 3,
                    is_prime: true,
                },
                Direction::Left,
            ),
            (
                UlamValue {
                    value: 4,
                    is_prime: false,
                },
                Direction::Left,
            ),
            (
                UlamValue {
                    value: 5,
                    is_prime: true,
                },
                Direction::Down,
            ),
            (
                UlamValue {
                    value: 6,
                    is_prime: false,
                },
                Direction::Down,
            ),
            (
                UlamValue {
                    value: 7,
                    is_prime: true,
                },
                Direction::Right,
            ),
            (
                UlamValue {
                    value: 8,
                    is_prime: false,
                },
                Direction::Right,
            ),
            (
                UlamValue {
                    value: 9,
                    is_prime: false,
                },
                Direction::Right,
            ),
            (
                UlamValue {
                    value: 10,
                    is_prime: false,
                },
                Direction::Up,
            ),
        ];
        assert_eq!(expected, generated);
    }

    #[test]
    fn to_coordinates_should_generate_correct_spiral_of_two() {
        let generated = UlamGenerator::default()
            .into_iter()
            .take(2)
            .to_coordinates()
            .collect::<Vec<(UlamValue, (isize, isize))>>();

        let expected = vec![
            (
                UlamValue {
                    value: 1,
                    is_prime: false,
                },
                (0, 0),
            ),
            (
                UlamValue {
                    value: 2,
                    is_prime: true,
                },
                (0, 1),
            ),
        ];
        assert_eq!(expected, generated);
    }

    #[test]
    fn to_coordinates_should_generate_correct_spiral_of_four() {
        let generated = UlamGenerator::default()
            .into_iter()
            .take(4)
            .to_coordinates()
            .collect::<Vec<(UlamValue, (isize, isize))>>();

        let expected = vec![
            (
                UlamValue {
                    value: 1,
                    is_prime: false,
                },
                (0, 0),
            ),
            (
                UlamValue {
                    value: 2,
                    is_prime: true,
                },
                (0, 1),
            ),
            (
                UlamValue {
                    value: 3,
                    is_prime: true,
                },
                (-1, 1),
            ),
            (
                UlamValue {
                    value: 4,
                    is_prime: false,
                },
                (-1, 0),
            ),
        ];
        assert_eq!(expected, generated);
    }

    #[test]
    fn to_coordinates_should_generate_correct_spiral_of_10() {
        let generated = UlamGenerator::default()
            .into_iter()
            .take(10)
            .to_coordinates()
            .collect::<Vec<(UlamValue, (isize, isize))>>();

        let expected = vec![
            (
                UlamValue {
                    value: 1,
                    is_prime: false,
                },
                (0, 0),
            ),
            (
                UlamValue {
                    value: 2,
                    is_prime: true,
                },
                (0, 1),
            ),
            (
                UlamValue {
                    value: 3,
                    is_prime: true,
                },
                (-1, 1),
            ),
            (
                UlamValue {
                    value: 4,
                    is_prime: false,
                },
                (-1, 0),
            ),
            (
                UlamValue {
                    value: 5,
                    is_prime: true,
                },
                (-1, -1),
            ),
            (
                UlamValue {
                    value: 6,
                    is_prime: false,
                },
                (0, -1),
            ),
            (
                UlamValue {
                    value: 7,
                    is_prime: true,
                },
                (1, -1),
            ),
            (
                UlamValue {
                    value: 8,
                    is_prime: false,
                },
                (1, 0),
            ),
            (
                UlamValue {
                    value: 9,
                    is_prime: false,
                },
                (1, 1),
            ),
            (
                UlamValue {
                    value: 10,
                    is_prime: false,
                },
                (1, 2),
            ),
        ];
        assert_eq!(expected, generated);
    }
}
