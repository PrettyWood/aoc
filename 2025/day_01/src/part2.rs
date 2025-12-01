pub fn solve_part2(input: &str) -> usize {
    let (_pos, zeroes) = input.lines().fold((50, 0), |(pos, zeroes), line| {
        let (dir, value) = line.split_at(1);
        let m = if dir == "L" { -1 } else { 1 };
        let v: isize = value.parse().unwrap();

        let new_pos = pos + m * v;
        let to_add: usize = match new_pos.cmp(&pos) {
            std::cmp::Ordering::Greater => new_pos.div_euclid(100) - pos.div_euclid(100),
            std::cmp::Ordering::Less => (pos - 1).div_euclid(100) - (new_pos - 1).div_euclid(100),
            std::cmp::Ordering::Equal => 0,
        }
        .try_into()
        .expect("to_add should always be positive");

        (new_pos, zeroes + to_add)
    });

    zeroes
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
        6
    )]
    #[case("R1000", 10)]
    fn test_solve_part2(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part2(input), expected);
    }
}
