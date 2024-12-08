use advent_of_code::Point;
use itertools::Itertools;
use multimap::MultiMap;
use std::{collections::HashSet, str::FromStr};

advent_of_code::solution!(8);

#[derive(Debug)]
struct Grid {
    antennas: MultiMap<char, Point>,
    cols: usize,
    rows: usize,
}

impl Grid {
    fn point_inside(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.cols as isize && point.y >= 0 && point.y < self.rows as isize
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let antennas = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| match c {
                    '.' => None,
                    _ => Some((
                        c,
                        Point {
                            x: x as isize,
                            y: y as isize,
                        },
                    )),
                })
            })
            .collect();

        let rows = s.lines().count();
        let cols = s.lines().map(|l| l.len()).max().unwrap_or_default();

        Ok(Grid {
            antennas,
            rows,
            cols,
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid = input.parse().ok()?;
    let antinodes: HashSet<Point> = grid
        .antennas
        .iter_all()
        .flat_map(|(_, antennas)| {
            antennas.iter().tuple_combinations().flat_map(|(a, b)| {
                [
                    Point {
                        x: 2 * a.x - b.x,
                        y: 2 * a.y - b.y,
                    },
                    Point {
                        x: 2 * b.x - a.x,
                        y: 2 * b.y - a.y,
                    },
                ]
                .into_iter()
                .filter(|p| grid.point_inside(p))
            })
        })
        .collect();
    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
