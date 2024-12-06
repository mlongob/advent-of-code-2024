use itertools::Itertools;

advent_of_code::solution!(2);

type Report = Vec<u32>;

fn parse_report(l: &str) -> Report {
    l.split_ascii_whitespace().filter_map(|x| x.parse().ok()).collect()
}

fn validate(report: &Report) -> bool {
    let increasing = report [0] < report [1];
    report.iter().tuple_windows().all(|(prev, next) | {
        if (increasing && next <= prev) || (!increasing && next >= prev) {
            return false;
        }
        let diff = prev.abs_diff(*next);
        diff >=1 && diff <= 3
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.lines().map(parse_report).filter(|r|validate(r)).count())
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
