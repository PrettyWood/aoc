pub fn solve_part2(input: &str) -> usize {
    input.lines().map(parse_line).sum()
}

fn parse_line(line: &str) -> usize {
    let line = transform_str_digits(line);
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

fn transform_str_digits(line: &str) -> String {
    line.chars()
        .enumerate()
        .map(|(i, c)| get_number(&line[i..]).unwrap_or(c))
        .collect()
}

const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_number(string: &str) -> Option<char> {
    for (i, n) in NUMBERS.iter().enumerate() {
        if string.starts_with(n) {
            return Some(char::from_digit(i as u32, 10).expect("digit should be a valid char"));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    fn test_parse_line(#[case] line: &str, #[case] expected: usize) {
        assert_eq!(parse_line(line), expected);
    }

    #[test]
    fn test_solve_part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(solve_part2(input), 281);
    }
}
