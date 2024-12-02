pub fn solve_part1(input: &str) -> usize {
    input.lines().map(parse_line).sum()
}

fn parse_line(line: &str) -> usize {
    let first_digit = line
        .chars()
        .find(|&c| c.is_ascii_digit())
        .expect("should have at least one char");
    let last_digit = line
        .chars()
        .filter(|&c| c.is_ascii_digit())
        .last()
        .expect("should have at least one char");
    format!("{first_digit}{last_digit}")
        .parse::<usize>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    fn test_parse_line(#[case] line: &str, #[case] expected: usize) {
        assert_eq!(parse_line(line), expected);
    }

    #[test]
    fn test_solve_part1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(solve_part1(input), 142);
    }
}
