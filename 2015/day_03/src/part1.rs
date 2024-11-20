use std::collections::HashSet;

pub fn solve_part1(input: &str) -> usize {
    input
        .chars()
        .fold(
            (HashSet::from([(0_isize, 0_isize)]), (0_isize, 0_isize)),
            |(mut visited_positions, prev_pos), c| {
                let new_position = match c {
                    '>' => (prev_pos.0 + 1, prev_pos.1),
                    '<' => (prev_pos.0 - 1, prev_pos.1),
                    '^' => (prev_pos.0, prev_pos.1 + 1),
                    'v' => (prev_pos.0, prev_pos.1 - 1),
                    _ => panic!("invalid char ${c}"),
                };
                visited_positions.insert(new_position);
                (visited_positions, new_position)
            },
        )
        .0
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(">", 2)]
    #[case("^>v<", 4)]
    #[case("^v^v^v^v^v", 2)]
    fn test_solve_part1(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part1(input), expected);
    }
}
