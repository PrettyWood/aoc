use std::collections::HashSet;

pub fn solve_part2(input: &str) -> usize {
    let (before_after_input, updates_input) = input
        .split_once("\n\n")
        .expect("there should be two blocks in input");
    let before_after_map = parse_before_after_map(before_after_input);

    updates_input
        .lines()
        .filter_map(|update| middle_if_invalid_update(update, &before_after_map))
        .sum()
}

fn middle_if_invalid_update(
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

    reorder_if_invalid(&numbers, before_after_map).map(|reordered| {
        assert!(reordered.len() % 2 == 1);
        reordered[(reordered.len() - 1) / 2]
    })
}

fn reorder_if_invalid(
    input: &[usize],
    before_after_map: &HashSet<(usize, usize)>,
) -> Option<Vec<usize>> {
    // vec![97, 13, 75, 29, 47] -> vec![97, 75, 47, 29, 13]
    let mut output = input.to_vec();
    let mut is_invalid = false;

    for i in 1..input.len() {
        for j in 0..i {
            if before_after_map.contains(&(output[i], output[j])) {
                output.swap(i, j);
                is_invalid = true;
            }
        }
    }
    is_invalid.then_some(output)
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

    use rstest::{fixture, rstest};

    #[fixture]
    fn before_after_map() -> HashSet<(usize, usize)> {
        parse_before_after_map(
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
        )
    }

    #[rstest]
    #[case(vec![75,47,61,53,29], None)]
    #[case(vec![75,97,47,61,53], Some(vec![97,75,47,61,53]))]
    #[case(vec![61,13,29], Some(vec![61,29,13]))]
    #[case(vec![97,13,75,29,47], Some(vec![97,75,47,29,13]))]
    fn test_reorder_if_invalid(
        #[case] input: Vec<usize>,
        #[case] expected: Option<Vec<usize>>,
        before_after_map: HashSet<(usize, usize)>,
    ) {
        assert_eq!(reorder_if_invalid(&input, &before_after_map), expected);
    }

    #[rstest]
    #[case("75,47,61,53,29", None)]
    #[case("97,61,53,29,13", None)]
    #[case("75,29,13", None)]
    #[case("75,97,47,61,53", Some(47))]
    #[case("61,13,29", Some(29))]
    #[case("97,13,75,29,47", Some(47))]
    fn test_middle_if_invalid_update(
        #[case] update: &str,
        #[case] middle_if_invalid: Option<usize>,
        before_after_map: HashSet<(usize, usize)>,
    ) {
        assert_eq!(
            middle_if_invalid_update(update, &before_after_map),
            middle_if_invalid
        );
    }

    #[test]
    fn test_solve_part2() {
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
        assert_eq!(solve_part2(input), 123);
    }
}
