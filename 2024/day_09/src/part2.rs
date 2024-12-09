use std::marker::PhantomData;

pub fn solve_part2(input: &str) -> usize {
    Blocks::from(input).arrange().hash()
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Block {
    File(usize),
    Free,
}

impl Block {
    fn has_id(&self, id: usize) -> bool {
        match self {
            Block::File(n) => *n == id,
            Block::Free => false,
        }
    }
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

        let last_id = match output.last() {
            Some(Block::File(id)) => *id,
            _ => panic!("it should end with a file"),
        };

        for id in (0..=last_id).rev() {
            let id_slice_start = output
                .iter()
                .position(|b| b.has_id(id))
                .expect("there should be a block with expected id");
            let id_slice_len = output
                .iter()
                .skip(id_slice_start)
                .take_while(|b| b.has_id(id))
                .count();

            for j in 0..id_slice_start {
                if output
                    .iter()
                    .skip(j)
                    .take(id_slice_len)
                    .all(|b| *b == Block::Free)
                {
                    for x in 0..id_slice_len {
                        output.swap(j + x, id_slice_start + x)
                    }
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
        self.blocks.iter().enumerate().fold(0, |acc, (i, b)| {
            if let Block::File(id) = b {
                acc + i * id
            } else {
                acc
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part2() {
        let input = "2333133121414131402";
        assert_eq!(solve_part2(input), 2858);
    }
}
