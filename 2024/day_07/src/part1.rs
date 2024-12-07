use rayon::prelude::*;

pub fn solve_part1(input: &str) -> usize {
    input
        .par_lines()
        .map(parse_line)
        .filter_map(|(values, total)| is_solvable(&values, total).then_some(total))
        .sum::<usize>()
}

fn parse_line(line: &str) -> (Vec<usize>, usize) {
    let (res_str, values_str) = line.split_once(": ").unwrap();
    (
        values_str
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect(),
        res_str.parse::<usize>().unwrap(),
    )
}

fn is_solvable(values: &[usize], total: usize) -> bool {
    match values {
        [] => total == 0,
        [v] => *v == total,
        [v1, v2, rest @ ..] => {
            is_solvable(&[&[v1 + v2], rest].concat(), total)
                || is_solvable(&[&[v1 * v2], rest].concat(), total)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(0, &[], true)]
    #[case(1, &[], false)]
    #[case(190, &[190], true)]
    #[case(190, &[180], false)]
    #[case(190, &[10, 19], true)]
    #[case(3267, &[81, 40, 27], true)]
    #[case(83, &[17, 5], false)]
    #[case(156, &[15, 6], false)]
    #[case(7290, &[6, 8, 6, 15], false)]
    #[case(161011, &[16, 10, 13], false)]
    #[case(192, &[17, 8, 14], false)]
    #[case(21037, &[9, 7, 18, 13], false)]
    #[case(292, &[11, 6, 16, 20], true)]
    fn test_is_solvable(#[case] expected: usize, #[case] values: &[usize], #[case] solvable: bool) {
        assert_eq!(is_solvable(values, expected), solvable)
    }

    #[test]
    fn test_solve_part1() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(solve_part1(input), 3749);
    }
}
