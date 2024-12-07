use regex::Regex;

advent_of_code::solution!(3);

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Mul(u32, u32),
}

type Program = Vec<Instruction>;

fn parse_program(input: &str) -> Option<Program> {
    let re = Regex::new(r"(?:(mul)\(([0-9]{1,3}),([0-9]{1,3})\))|(?:(do|don't)\(()()\))").ok()?;
    Some(
        re.captures_iter(input)
            .filter_map(|caps| {
                let (_, [inst, lhs, rhs]) = caps.extract();
                match inst {
                    "mul" => Some(Instruction::Mul(lhs.parse().ok()?, rhs.parse().ok()?)),
                    "do" => Some(Instruction::Do),
                    "don't" => Some(Instruction::Dont),
                    _ => None,
                }
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
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
