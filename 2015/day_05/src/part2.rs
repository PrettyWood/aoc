use std::collections::HashMap;

pub fn solve_part2(input: &str) -> usize {
    input.lines().filter(|line| is_nice_string(line)).count()
}

fn is_nice_string(string: &str) -> bool {
    let mut seen_pairs: HashMap<(char, char), usize> = HashMap::new();
    let has_two_non_overlapping_pairs = string.chars().zip(string.chars().skip(1)).enumerate().any(
        |(i, (c1, c2))| match seen_pairs.get(&(c1, c2)) {
            None => {
                seen_pairs.insert((c1, c2), i);
                false
            }
            Some(other_pos) => i - other_pos >= 2,
        },
    );

    let has_two_same_letters_with_one_between = string
        .chars()
        .zip(string.chars().skip(2))
        .any(|(c1, c2)| c1 == c2);

    has_two_non_overlapping_pairs && has_two_same_letters_with_one_between
}

#[cfg(test)]
mod tests {
    use super::is_nice_string;
    use rstest::rstest;

    #[rstest]
    #[case("qjhvhtzxzqqjkmpb", true)]
    #[case("xxyxx", true)]
    #[case("uurcxstgmygtbstg", false)]
    #[case("ieodomkazucvgmuy", false)]
    fn test_is_nice(#[case] string: &str, #[case] is_nice: bool) {
        assert_eq!(is_nice_string(string), is_nice)
    }
}
