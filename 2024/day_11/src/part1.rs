pub fn solve_part1(input: &str) -> usize {
    let stones = Stones::from(input);
    std::iter::successors(Some(stones), |s| Some(s.next()))
        .nth(25)
        .expect("there should be stones!")
        .len()
}

#[derive(Debug)]
struct Stones(Vec<Stone>);

impl Stones {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn next(&self) -> Stones {
        Stones(self.0.iter().flat_map(|stone| stone.next()).collect())
    }
}

impl From<&str> for Stones {
    fn from(input: &str) -> Self {
        Stones(input.split_whitespace().map(|l| l.into()).collect())
    }
}

#[derive(Debug)]
struct Stone(usize);

impl Stone {
    fn next(&self) -> Vec<Stone> {
        let s = self.0.to_string();
        match self.0 {
            0 => vec![Stone(1)],
            _ if s.len() % 2 == 0 => {
                let (left, right) = s.split_at(s.len() / 2);
                vec![
                    Stone(left.parse::<usize>().unwrap()),
                    Stone(right.parse::<usize>().unwrap()),
                ]
            }
            v => vec![Stone(v * 2024)],
        }
    }
}

impl From<&str> for Stone {
    fn from(value: &str) -> Self {
        Stone(
            value
                .parse::<usize>()
                .expect("value should be a valid usize"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1("125 17"), 55312);
    }
}
