use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_line(line: &str) -> Option<(u32, u32)> {
    let mut values = line.split_ascii_whitespace().filter_map(|x| x.parse().ok());
    let left = values.next()?;
    let right = values.next()?;
    Some((left, right))
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input.lines().filter_map(parse_line).unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();
    Some(
        left.into_iter()
            .zip(right)
            .fold(0, |acc, (l, r)| acc + l.abs_diff(r)),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_input(input);

    let counts = right.into_iter().fold(HashMap::new(), |mut acc, val| {
        *acc.entry(val).or_default() += 1;
        acc
    });
    Some(left.iter().map(|x| x * counts.get(x).unwrap_or(&0)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
