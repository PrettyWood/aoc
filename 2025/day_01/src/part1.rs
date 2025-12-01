pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .scan(50, |pos, line| {
            let (dir, value) = line.split_at(1);
            let m = if dir == "L" { -1 } else { 1 };
            let v: isize = value.parse().unwrap();
            *pos = (*pos + m * v).rem_euclid(100);
            Some(*pos)
        })
        .filter(|pos| *pos == 0)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        3
    )]
    fn test_solve_part1(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part1(input), expected);
    }
}
