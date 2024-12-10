use pathfinding::prelude::count_paths;

pub fn solve_part2(input: &str) -> usize {
    let map: Map = input.into();
    map.rating()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug)]
struct Map {
    n_rows: usize,
    n_cols: usize,
    positions: Vec<Pos>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let n_rows = input.lines().count();
        let n_cols = input
            .lines()
            .next()
            .expect("there should be at least one line")
            .chars()
            .count();
        let positions = input
            .lines()
            .enumerate()
            .flat_map(|(x, line)| {
                line.chars().enumerate().flat_map(move |(y, c)| {
                    c.to_digit(10).map(|z| Pos {
                        x,
                        y,
                        z: z as usize,
                    })
                })
            })
            .collect();
        Self {
            n_rows,
            n_cols,
            positions,
        }
    }
}

impl Map {
    fn starts(&self) -> Vec<Pos> {
        self.positions
            .iter()
            .filter(|pos| pos.z == 0)
            .cloned()
            .collect()
    }

    fn ends(&self) -> Vec<Pos> {
        self.positions
            .iter()
            .filter(|pos| pos.z == 9)
            .cloned()
            .collect()
    }

    fn pos(&self, x: isize, y: isize) -> Option<&Pos> {
        if x < 0 || y < 0 || x > self.n_rows as isize || y > self.n_cols as isize {
            return None;
        }
        self.positions
            .iter()
            .find(|pos| pos.x == x as usize && pos.y == y as usize)
    }

    fn neighbours(&self, pos: &Pos) -> Vec<Pos> {
        let x = pos.x as isize;
        let y = pos.y as isize;
        [
            self.pos(x - 1, y),
            self.pos(x + 1, y),
            self.pos(x, y - 1),
            self.pos(x, y + 1),
        ]
        .iter()
        .filter_map(|c| c.cloned())
        .filter(|c| c.z == pos.z + 1)
        .collect()
    }

    fn rating(&self) -> usize {
        self.starts()
            .into_iter()
            .flat_map(|start| self.ends().into_iter().map(move |end| (start.clone(), end)))
            .map(|(start, end)| count_paths(start.clone(), |p| self.neighbours(p), |p| *p == end))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....",
        3
    )]
    #[case(
        "..90..9
...1.98
...2..7
6543456
765.987
876....
987....",
        13
    )]
    #[case(
        "012345
123456
234567
345678
4.6789
56789.",
        227
    )]
    #[case(
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        81
    )]
    fn test_solve_part2(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part2(input), expected);
    }
}
