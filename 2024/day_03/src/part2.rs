use regex::Regex;

pub fn solve_part2(input: &str) -> usize {
    let re =
        Regex::new(r"mul\((?P<n1>\d+),(?P<n2>\d+)\)|(?P<do>do\(\))|(?P<dont>don't\(\))").unwrap();
    re.captures_iter(input)
        .fold((0, true), |(total, enabled), c| {
            if c.name("do").is_some() {
                (total, true)
            } else if c.name("dont").is_some() {
                (total, false)
            } else if enabled {
                let a = c.name("n1").unwrap().as_str().parse::<usize>().unwrap();
                let b = c.name("n2").unwrap().as_str().parse::<usize>().unwrap();
                (total + a * b, enabled)
            } else {
                (total, enabled)
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(solve_part2(input), 48);
    }
}
