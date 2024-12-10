use pathfinding::prelude::bfs;

pub fn solve_part1(input: &str) -> usize {
    let map: Map = input.into();
    map.score()
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

    fn score(&self) -> usize {
        self.starts()
            .into_iter()
            .flat_map(|start| self.ends().into_iter().map(move |end| (start.clone(), end)))
            .filter(|(start, end)| bfs(start, |p| self.neighbours(p), |p| p == end).is_some())
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        "0123
1234
8765
9876",
        1
    )]
    #[case(
        "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9",
        2
    )]
    #[case(
        "..90..9
...1.98
...2..7
6543456
765.987
876....
987....",
        4
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
        36
    )]
    fn test_solve_part1(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part1(input), expected);
    }
}
