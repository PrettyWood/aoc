use std::collections::HashSet;

pub fn solve_part2(input: &str) -> usize {
    input
        .chars()
        .fold(
            (
                HashSet::from([(0_isize, 0_isize)]),
                (0_isize, 0_isize),
                (0_isize, 0_isize),
                true,
            ),
            |(mut visited_positions, mut santa_pos, mut robo_pos, santa_turn), c| {
                if santa_turn {
                    santa_pos = get_new_position(c, santa_pos);
                    visited_positions.insert(santa_pos);
                } else {
                    robo_pos = get_new_position(c, robo_pos);
                    visited_positions.insert(robo_pos);
                }
                (visited_positions, santa_pos, robo_pos, !santa_turn)
            },
        )
        .0
        .len()
}

fn get_new_position(c: char, prev_pos: (isize, isize)) -> (isize, isize) {
    match c {
        '>' => (prev_pos.0 + 1, prev_pos.1),
        '<' => (prev_pos.0 - 1, prev_pos.1),
        '^' => (prev_pos.0, prev_pos.1 + 1),
        'v' => (prev_pos.0, prev_pos.1 - 1),
        _ => panic!("invalid char ${c}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("^v", 3)]
    #[case("^>v<", 3)]
    #[case("^v^v^v^v^v", 11)]
    fn test_solve_part2(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part2(input), expected);
    }
}
