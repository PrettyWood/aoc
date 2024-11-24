pub fn solve_part1(input: &str) -> usize {
    input.lines().filter(|line| is_nice_string(line)).count()
}

fn is_nice_string(string: &str) -> bool {
    let countains_three_vowels = string
        .chars()
        .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
        >= 3;

    let has_same_char_in_a_row = string
        .chars()
        .zip(string.chars().skip(1))
        .any(|(c1, c2)| c1 == c2);

    let has_forbidden_seq = ["ab", "cd", "pq", "xy"]
        .iter()
        .any(|seq| string.contains(seq));

    countains_three_vowels && has_same_char_in_a_row && !has_forbidden_seq
}

#[cfg(test)]
mod tests {
    use super::is_nice_string;
    use rstest::rstest;

    #[rstest]
    #[case("ugknbfddgicrmopn", true)]
    #[case("aaa", true)]
    #[case("jchzalrnumimnmhp", false)]
    #[case("haegwjzuvuyypxyu", false)]
    #[case("dvszwmarrgswjxmb", false)]
    fn test_is_nice(#[case] string: &str, #[case] is_nice: bool) {
        assert_eq!(is_nice_string(string), is_nice)
    }
}
