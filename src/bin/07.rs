advent_of_code::solution!(7);

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .filter_map(|l| {
            let (l, r) = l.split_once(":")?;
            let res: u64 = l.parse().ok()?;
            let ops: Vec<u64> = r
                .split_ascii_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            Some((res, ops))
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    dbg!(parse(input));
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
