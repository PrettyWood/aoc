use std::collections::HashSet;

pub fn solve_part1(input: &str) -> usize {
    let (before_after_input, updates_input) = input
        .split_once("\n\n")
        .expect("there should be two blocks in input");
    let before_after_map = parse_before_after_map(before_after_input);

    updates_input
        .lines()
        .filter_map(|update| middle_if_valid_update(update, &before_after_map))
        .sum()
}

fn middle_if_valid_update(
    update: &str,
    before_after_map: &HashSet<(usize, usize)>,
) -> Option<usize> {
    let numbers: Vec<_> = update
        .split(',')
        .map(|n| {
            n.parse::<usize>()
                .expect("update should contain only numbers")
        })
        .collect();

    let is_valid = numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .all(|(&n1, &n2)| !before_after_map.contains(&(n2, n1)));

    assert!(numbers.len() % 2 == 1);
    is_valid.then_some(numbers[(numbers.len() - 1) / 2])
}

fn parse_before_after_map(input: &str) -> HashSet<(usize, usize)> {
    input
        .lines()
        .map(|l| {
            let (before, after) = l.split_once('|').expect("there should be a | in orders");
            let before = before
                .parse::<usize>()
                .expect("before should be a valid number");
            let after = after
                .parse::<usize>()
                .expect("after should be a valid number");
            (before, after)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("75,47,61,53,29", Some(61))]
    #[case("97,61,53,29,13", Some(53))]
    #[case("75,29,13", Some(29))]
    #[case("75,97,47,61,53", None)]
    #[case("61,13,29", None)]
    #[case("97,13,75,29,47", None)]
    fn test_middle_if_valid_update(#[case] update: &str, #[case] middle_if_valid: Option<usize>) {
        let before_after_map = parse_before_after_map(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13",
        );
        assert_eq!(
            middle_if_valid_update(update, &before_after_map),
            middle_if_valid
        );
    }

    #[test]
    fn test_solve_part1() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(solve_part1(input), 143);
    }
}
