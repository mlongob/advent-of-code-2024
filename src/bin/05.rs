use std::collections::HashMap;

advent_of_code::solution!(5);

type Orders = HashMap<u32, Vec<u32>>;
type Update = Vec<u32>;
pub fn parse_input(input: &str) -> Option<(Orders, Vec<Update>)> {
    let (header, footer) = input.split_once("\n\n")?;
    let orders: Orders = header
        .lines()
        .filter_map(|l| {
            let (lhs, rhs) = l.split_once("|")?;
            Some((lhs.parse().ok()?, rhs.parse().ok()?))
        })
        .fold(HashMap::new(), |mut acc, (pre, post)| {
            acc.entry(post).or_default().push(pre);
            acc
        });

    let updates: Vec<Update> = footer
        .lines()
        .map(|l| l.split(",").filter_map(|e| e.parse().ok()).collect())
        .collect();
    Some((orders, updates))
}

fn validate_update(orders: &Orders, update: &Update) -> bool {
    for (i, x) in update.iter().enumerate() {
        let rest = &update[i..];

        if let Some(pre) = orders.get(x) {
            if rest.iter().any(|y| pre.contains(y)) {
                return false;
            }
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let (orders, updates) = parse_input(input)?;
    Some(
        updates
            .into_iter()
            .filter(|update| validate_update(&orders, update))
            .map(|update| update[update.len() / 2])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (orders, updates) = parse_input(input)?;
    Some(
        updates
            .into_iter()
            .filter(|update| !validate_update(&orders, update))
            .map(|mut update| {
                update.sort_unstable_by(|a, b| {
                    if orders.get(b).map(|v| v.contains(a)).unwrap_or_default() {
                        core::cmp::Ordering::Greater
                    } else if orders.get(a).map(|v| v.contains(b)).unwrap_or_default() {
                        core::cmp::Ordering::Less
                    } else {
                        core::cmp::Ordering::Equal
                    }
                });
                update
            })
            .map(|update| update[update.len() / 2])
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
