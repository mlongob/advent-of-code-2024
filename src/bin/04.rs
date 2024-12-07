use itertools::{iproduct, Itertools};
use std::str::FromStr;

use advent_of_code::{Direction, Point};

advent_of_code::solution!(4);

#[derive(Debug)]
pub struct Matrix {
    pub cells: Vec<Vec<char>>,
    pub cols: usize,
    pub rows: usize,
}

const CHARS: [char; 4] = ['X', 'M', 'A', 'S'];

impl Matrix {
    fn get(&self, point: &Point) -> char {
        self.cells[point.y as usize][point.x as usize]
    }

    fn safe_get(&self, point: &Point) -> Option<char> {
        if self.point_inside(point) {
            Some(self.cells[point.y as usize][point.x as usize])
        } else {
            None
        }
    }

    fn point_inside(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.cols as isize && point.y >= 0 && point.y < self.rows as isize
    }

    fn starts(&self) -> Vec<Point> {
        iproduct!(0..self.cols, 0..self.rows)
            .filter_map(|(x, y)| {
                let point = Point {
                    x: x as isize,
                    y: y as isize,
                };
                const START: char = CHARS[0];
                match self.get(&point) {
                    START => Some(point),
                    _ => None,
                }
            })
            .collect()
    }
}

impl FromStr for Matrix {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells: Vec<Vec<char>> = s
            .lines()
            .filter_map(|line| {
                if line.is_empty() {
                    None
                } else {
                    Some(line.chars().collect())
                }
            })
            .collect();

        let rows = cells.len();
        let cols = cells[0].len();

        Ok(Matrix { cells, rows, cols })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let matrix: Matrix = input.parse().ok()?;
    Some(
        matrix
            .starts()
            .into_iter()
            .cartesian_product(Direction::ALL_DIRECTIONS)
            .filter(|(start, direction)| {
                CHARS
                    .iter()
                    .try_fold(*start, |point, c| {
                        if matrix.safe_get(&point)? != *c {
                            None
                        } else {
                            Some(point.neighbor(*direction))
                        }
                    })
                    .is_some()
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
