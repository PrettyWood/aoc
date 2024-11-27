use chumsky::prelude::*;

pub fn solve_part1(input: &str) -> usize {
    let instruction_parser = instruction_parser();
    let instructions = input
        .lines()
        .map(|line| {
            instruction_parser
                .parse(line)
                .expect("should be a valid instruction")
        })
        .collect::<Vec<_>>();

    let mut grid = [[false; 1000]; 1000];
    for instruction in &instructions {
        for i in (instruction.from.0)..=(instruction.to.0) {
            for j in (instruction.from.1)..=(instruction.to.1) {
                grid[i][j] = match instruction.command {
                    Command::TurnOn => true,
                    Command::TurnOff => false,
                    Command::Toggle => !grid[i][j],
                };
            }
        }
    }

    grid.iter().flatten().filter(|light| **light).count()
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

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("turn on 887,9 through 959,629", Instruction {
        command: Command::TurnOn,
        from: (887, 9),
        to: (959, 629),
    })]
    #[case("toggle 753,664 through 970,926", Instruction {
        command: Command::Toggle,
        from: (753,664),
        to: (970,926),
    })]
    #[case("turn off 150,300 through 213,740", Instruction {
        command: Command::TurnOff,
        from: (150, 300),
        to: (213, 740),
    })]
    fn test_instruction_parser(#[case] input: &str, #[case] expected_instruction: Instruction) {
        assert_eq!(
            instruction_parser().parse(input).unwrap(),
            expected_instruction
        )
    }
}
