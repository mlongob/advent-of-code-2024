
use regex::Regex;

advent_of_code::solution!(3);

enum Instruction {
    Do,
    Dont,
    Mul(u32, u32),
}

type Program = Vec<Instruction>;

fn parse_program(input: &str) -> Option<Program> {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").ok()?;
    Some(
        re.captures_iter(input)
            .filter_map(|caps| {
                let (_, [lhs_s, rhs_s]) = caps.extract();
                let lhs: u32 = lhs_s.parse().ok()?;
                let rhs: u32 = rhs_s.parse().ok()?;
                Some(Instruction::Mul(lhs, rhs))
            })
            .collect(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_program(input)?
            .into_iter()
            .filter_map(|inst| match inst {
                Instruction::Mul(lhs, rhs) => Some(lhs * rhs),
                _ => None,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse_program(input)?
            .into_iter()
            .fold((true, 0), |(enabled, sum), inst| match (inst, enabled) {
                (Instruction::Do, _) => (true, sum),
                (Instruction::Dont, _) => (false, sum),
                (Instruction::Mul(lhs, rhs), true) => (enabled, sum + lhs * rhs),
                (_, false) => (false, sum),
            })
            .1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
