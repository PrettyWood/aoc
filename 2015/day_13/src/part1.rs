use std::collections::{HashMap, HashSet};

use chumsky::prelude::*;
use itertools::Itertools;
use text::newline;

pub fn solve_part1(input: &str) -> isize {
    let instructions = parser().parse(input).unwrap();

    let happiness_map: HashMap<(String, String), isize> = instructions
        .iter()
        .map(|instr| ((instr.name.clone(), instr.next_to.clone()), instr.happiness))
        .collect();

    let all_names: HashSet<String> = instructions
        .iter()
        .map(|instr| instr.name.clone())
        .collect();

    all_names
        .iter()
        .permutations(all_names.len())
        .map(|perm| {
            perm.into_iter()
                .circular_tuple_windows::<(_, _)>()
                .map(|(n1, n2)| {
                    let h1 = happiness_map
                        .get(&(n1.to_string(), n2.to_string()))
                        .expect("h1 should be defined");
                    let h2 = happiness_map
                        .get(&(n2.to_string(), n1.to_string()))
                        .expect("h2 should be defined");
                    h1 + h2
                })
                .sum()
        })
        .max()
        .expect("max should exist")
}

#[derive(Debug)]
struct Instruction {
    name: String,
    happiness: isize,
    next_to: String,
}

fn parser() -> impl Parser<char, Vec<Instruction>, Error = Simple<char>> {
    instruction_parser().separated_by(newline())
}

// Alice would lose 62 happiness units by sitting next to Carol.
// Alice would gain 65 happiness units by sitting next to David.
fn instruction_parser() -> impl Parser<char, Instruction, Error = Simple<char>> {
    text::ident()
        .then_ignore(just(" would "))
        .then(choice((
            just("gain ").then(usize_parser()).map(|(_, u)| u as isize),
            just("lose ")
                .then(usize_parser())
                .map(|(_, u)| -(u as isize)),
        )))
        .then_ignore(just(" happiness units by sitting next to "))
        .then(text::ident())
        .then_ignore(just("."))
        .map(|((name, happiness), next_to)| Instruction {
            name,
            happiness,
            next_to,
        })
}

fn usize_parser() -> impl Parser<char, usize, Error = Simple<char>> {
    text::int(10).map(|s: String| s.parse::<usize>().expect("should be a valid usize"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";
        assert_eq!(solve_part1(input), 330);
    }
}
