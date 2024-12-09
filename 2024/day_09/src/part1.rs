use std::marker::PhantomData;

pub fn solve_part1(input: &str) -> usize {
    Blocks::from(input).arrange().hash()
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Block {
    File(usize),
    Free,
}

#[derive(Debug, PartialEq, Eq)]
struct Raw;

#[derive(Debug, PartialEq, Eq)]
struct Arranged;

#[derive(Debug, PartialEq, Eq)]
struct Blocks<State> {
    blocks: Vec<Block>,
    state: PhantomData<State>,
}

impl From<&str> for Blocks<Raw> {
    fn from(input: &str) -> Self {
        Blocks {
            blocks: input
                .chars()
                .enumerate()
                .fold(
                    (Vec::with_capacity(input.len()), 0),
                    |(mut output, file_id), (i, c)| {
                        let n = c.to_digit(10).expect("should be a valid digit") as usize;
                        if i % 2 == 0 {
                            output.extend(std::iter::repeat(Block::File(file_id)).take(n));
                            (output, file_id + 1)
                        } else {
                            output.extend(std::iter::repeat(Block::Free).take(n));
                            (output, file_id)
                        }
                    },
                )
                .0,
            state: PhantomData::<Raw>,
        }
    }
}

impl Blocks<Raw> {
    fn arrange(&self) -> Blocks<Arranged> {
        let mut output: Vec<Block> = self.blocks.clone();
        let mut i = 0;
        let mut j = output.len() - 1;
        while i < j {
            match (&output[i], &output[j]) {
                (Block::File(_), _) => i += 1,
                (_, Block::Free) => j -= 1,
                (Block::Free, Block::File(_)) => {
                    output.swap(i, j);
                }
            }
        }

        Blocks {
            blocks: output,
            state: PhantomData::<Arranged>,
        }
    }
}

impl Blocks<Arranged> {
    fn hash(&self) -> usize {
        self.blocks
            .iter()
            .filter_map(|b| {
                if let Block::File(id) = b {
                    Some(id)
                } else {
                    None
                }
            })
            .enumerate()
            .fold(0, |acc, (i, id)| acc + i * id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("12345", Blocks {
        blocks: vec![
            Block::File(0),
            Block::Free,
            Block::Free,
            Block::File(1),
            Block::File(1),
            Block::File(1),
            Block::Free,
            Block::Free,
            Block::Free,
            Block::Free,
            Block::File(2),
            Block::File(2),
            Block::File(2),
            Block::File(2),
            Block::File(2),
        ],
        state: PhantomData::<Raw>,
    })]
    fn test_as_blocks(#[case] input: &str, #[case] expected: Blocks<Raw>) {
        assert_eq!(Blocks::from(input), expected);
    }

    #[rstest]
    #[case("12345", Blocks {
        blocks: vec![
        Block::File(0),
        Block::File(2),
        Block::File(2),
        Block::File(1),
        Block::File(1),
        Block::File(1),
        Block::File(2),
        Block::File(2),
        Block::File(2),
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Free,
    ], state: PhantomData::<Arranged> })]
    fn test_order_blocks(#[case] input: &str, #[case] expected: Blocks<Arranged>) {
        assert_eq!(Blocks::from(input).arrange(), expected);
    }

    #[test]
    fn test_solve_part1() {
        let input = "2333133121414131402";
        assert_eq!(solve_part1(input), 1928);
    }
}
