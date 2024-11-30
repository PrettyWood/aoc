use chumsky::prelude::*;

pub fn solve_part2(input: &str) -> usize {
    let instruction_parser = instruction_parser();
    let instructions = input
        .lines()
        .map(|line| {
            instruction_parser
                .parse(line)
                .expect("should be a valid instruction")
        })
        .collect::<Vec<_>>();

    let mut grid = [[0_usize; 1000]; 1000];
    for instruction in &instructions {
        for row in grid
            .iter_mut()
            .skip(instruction.from.0)
            .take(instruction.to.0 - instruction.from.0 + 1)
        {
            for cell in row
                .iter_mut()
                .skip(instruction.from.1)
                .take(instruction.to.1 - instruction.from.1 + 1)
            {
                *cell = match instruction.command {
                    Command::TurnOn => *cell + 1,
                    Command::TurnOff => (*cell).saturating_sub(1),
                    Command::Toggle => *cell + 2,
                };
            }
        }
    }

    grid.iter().map(|row| row.iter().sum::<usize>()).sum()
}

#[derive(Debug, PartialEq)]
struct Instruction {
    command: Command,
    from: (usize, usize),
    to: (usize, usize),
}

#[derive(Debug, Clone, PartialEq)]
enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

fn instruction_parser() -> impl Parser<char, Instruction, Error = Simple<char>> {
    command_parser()
        .then_ignore(just(" "))
        .then(range_parser())
        .then_ignore(just(" through "))
        .then(range_parser())
        .map(|((command, from), to)| Instruction { command, from, to })
}

fn command_parser() -> impl Parser<char, Command, Error = Simple<char>> {
    choice((
        just("turn on").to(Command::TurnOn),
        just("turn off").to(Command::TurnOff),
        just("toggle").to(Command::Toggle),
    ))
}

fn range_parser() -> impl Parser<char, (usize, usize), Error = Simple<char>> {
    usize_parser()
        .then_ignore(just(","))
        .then(usize_parser())
        .map(|(a, b)| (a, b))
}

fn usize_parser() -> impl Parser<char, usize, Error = Simple<char>> {
    text::int(10).map(|s: String| s.parse::<usize>().expect("should be a valid usize"))
}
