mod part1;
mod part2;

use part1::solve_part1;
use part2::solve_part2;

fn main() {
    println!("part 1: {}", solve_part1(include_str!("input1.txt")));
    println!("part 2: {}", solve_part2(include_str!("input2.txt")));
}
