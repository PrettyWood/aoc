pub fn solve_part1(input: &str) -> usize {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();
    let ranges: Vec<_> = ranges
        .lines()
        .map(|l| {
            let (start, end) = l.split_once('-').unwrap();
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();
            start..=end
        })
        .collect();

    ingredients
        .lines()
        .map(|l| l.parse().unwrap())
        .filter(|i| ranges.iter().any(|r| r.contains(i)))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        3
    )]
    fn test_solve_part1(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part1(input), expected);
    }
}
