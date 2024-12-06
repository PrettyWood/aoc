use std::{collections::HashSet, iter};

pub fn solve_part1(input: &str) -> usize {
    let grid_state: Vec<Vec<(Position, char)>> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| ((i as isize, j as isize), c))
                .collect()
        })
        .collect();
    let grid = Grid {
        n_rows: grid_state.len(),
        n_cols: grid_state[0].len(),
        obstacles: grid_state
            .iter()
            .flatten()
            .filter_map(|&(pos, c)| (c == '#').then_some(pos))
            .collect(),
    };
    let guard = Guard {
        pos: grid_state
            .iter()
            .flatten()
            .find_map(|&(pos, c)| (c == '^').then_some(pos))
            .expect("guard should be in grid"),
        direction: Direction::Up,
    };

    iter::successors(Some(guard), |g| g.next(&grid))
        .map(|g| g.pos)
        .collect::<HashSet<_>>()
        .len()
}

type Position = (isize, isize);

#[derive(Debug)]
struct Grid {
    n_rows: usize,
    n_cols: usize,
    obstacles: Vec<Position>,
}

impl Grid {
    fn is_inside(&self, pos: Position) -> bool {
        pos.0 >= 0 && pos.0 < self.n_rows as isize && pos.1 >= 0 && pos.1 < self.n_cols as isize
    }

    fn is_obstacle(&self, pos: Position) -> bool {
        self.obstacles.contains(&pos)
    }
}

#[derive(Debug)]
struct Guard {
    pub pos: Position,
    pub direction: Direction,
}

impl Guard {
    fn next(&self, grid: &Grid) -> Option<Guard> {
        let mut direction = self.direction.clone();
        let mut next_pos = direction.next_pos(self.pos);
        while grid.is_obstacle(next_pos) {
            direction = direction.turn();
            next_pos = direction.next_pos(self.pos);
        }

        grid.is_inside(next_pos).then_some(Guard {
            pos: next_pos,
            direction,
        })
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn next_pos(&self, pos: Position) -> Position {
        let (dx, dy) = match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        };
        (pos.0 + dx, pos.1 + dy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(solve_part1(input), 41);
    }
}
