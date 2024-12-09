pub fn solve_part1(input: &str) -> isize {
    let mut iter = input.chars().peekable();

    let mut numbers: Vec<isize> = Vec::new();
    while iter.peek().is_some() {
        if let Ok(n) = iter
            .by_ref()
            .skip_while(|c| !(c.is_ascii_digit() || *c == '-'))
            .take_while(|c| c.is_ascii_digit() || *c == '-')
            .collect::<String>()
            .parse::<isize>()
        {
            numbers.push(n);
        }
    }

    numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(r#"{"a":{"b":4},"c":-1}"#, 3)]
    #[case(r#"{"a":[-1,1]}"#, 0)]
    #[case(r#"{}"#, 0)]
    fn test_part_1(#[case] input: &str, #[case] expected: isize) {
        assert_eq!(solve_part1(input), expected);
    }
}
