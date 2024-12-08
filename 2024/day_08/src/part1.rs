use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn solve_part1(input: &str) -> usize {
    Grid::from(input).get_all_unique_antinodes().len()
}

#[derive(Debug)]
struct Grid {
    n_rows: usize,
    n_cols: usize,
    antennas: Antennas,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let lines: Vec<_> = input.lines().collect();
        let antennas: Antennas = lines
            .iter()
            .enumerate()
            .flat_map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .filter(|(_, c)| c.is_ascii_alphanumeric())
                    .map(move |(j, c)| (c, (i as isize, j as isize)))
            })
            .fold(Antennas::new(), |mut antennas, (fq, pos)| {
                antennas.entry(fq).or_default().push(pos);
                antennas
            });

        Grid {
            n_rows: lines.len(),
            n_cols: lines[0].len(),
            antennas,
        }
    }
}

impl Grid {
    fn is_inside(&self, (x, y): Position) -> bool {
        x >= 0 && x < self.n_rows as isize && y >= 0 && y < self.n_cols as isize
    }

    fn get_antinodes(&self, fq: &Frequency) -> HashSet<Position> {
        let positions = self.antennas.get(fq).expect("fq should be in map");
        positions
            .iter()
            .combinations(2)
            .flat_map(|pos1_pos2| {
                let (x1, y1) = *pos1_pos2[0];
                let (x2, y2) = *pos1_pos2[1];

                let dx = x1.abs_diff(x2) as isize;
                let dy = y1.abs_diff(y2) as isize;

                if x1 <= x2 && y1 <= y2 {
                    // ....A........
                    // ........B....
                    [(x1 - dx, y1 - dy), (x2 + dx, y2 + dy)]
                } else if x1 <= x2 && y1 > y2 {
                    // ........A....
                    // ....B........
                    [(x1 - dx, y1 + dy), (x2 + dx, y2 - dy)]
                } else if x1 > x2 && y1 <= y2 {
                    // ........B....
                    // ....A........
                    [(x1 + dx, y1 - dy), (x2 - dx, y2 + dy)]
                } else {
                    // ....B.......
                    // ........A...
                    [(x1 + dx, y1 + dy), (x2 - dx, y2 - dy)]
                }
            })
            .filter(|&pos| self.is_inside(pos))
            .collect()
    }

    fn get_all_unique_antinodes(&self) -> HashSet<Position> {
        self.antennas
            .keys()
            .fold(HashSet::new(), |mut unique_antinodes_pos, fq| {
                unique_antinodes_pos.extend(self.get_antinodes(fq));
                unique_antinodes_pos
            })
    }
}

type Position = (isize, isize);
type Frequency = char;
type Antennas = HashMap<Frequency, Vec<Position>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_antinodes() {
        let input = "..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......#...
..........
..........";
        let grid: Grid = input.into();
        let antinodes_pos: HashSet<Position> = input
            .lines()
            .enumerate()
            .flat_map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .filter_map(move |(j, c)| (c == '#').then_some((i as isize, j as isize)))
            })
            .collect();
        assert_eq!(grid.get_antinodes(&'a'.into()), antinodes_pos);
    }

    #[test]
    fn test_solve_part1() {
        let input = "......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.";
        assert_eq!(solve_part1(input), 14);
    }
}
