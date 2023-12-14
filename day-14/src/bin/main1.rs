use std::{fs, iter::repeat};

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", compute(input));
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Block {
    Cube(usize),
    LooseRoundies { round: usize, empty: usize },
}
impl Block {
    fn add(&self, new_block: Block) -> Option<Block> {
        match (self, new_block) {
            (
                Block::LooseRoundies {
                    round: round1,
                    empty: empty1,
                },
                Block::LooseRoundies {
                    round: round2,
                    empty: empty2,
                },
            ) => Some(Block::LooseRoundies {
                round: round1 + round2,
                empty: empty1 + empty2,
            }),
            (Block::Cube(n1), Block::Cube(n2)) => Some(Block::Cube(n1 + n2)),
            _ => None,
        }
    }
}

fn tilt(input: String) -> String {
    let cols = input.lines().skip(1).fold(
        input
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| vec![c])
            .collect::<Vec<_>>(),
        |acc, new_line| {
            acc.into_iter()
                .zip(new_line.chars())
                .map(|(mut old_line, new_char)| {
                    old_line.push(new_char);
                    old_line
                })
                .collect()
        },
    );

    let tilted_cols: Vec<_> = cols
        .into_iter()
        .map(|col| {
            // Establish blocks
            col.into_iter()
                .fold(vec![], |mut acc: Vec<Block>, c| {
                    let new_block = match c {
                        '.' => Block::LooseRoundies { round: 0, empty: 1 },
                        'O' => Block::LooseRoundies { round: 1, empty: 0 },
                        '#' => Block::Cube(1),
                        _ => panic!("Invalid input {:?}", c),
                    };

                    if let Some(last) = acc.last_mut() {
                        // Not the first
                        if let Some(maybe_addition) = last.add(new_block) {
                            *last = maybe_addition;
                        } else {
                            acc.push(new_block);
                        }
                        acc
                    } else {
                        vec![new_block]
                    }
                })
                .into_iter()
                .flat_map(|block| match block {
                    Block::Cube(n) => repeat('#').take(n).collect::<Vec<_>>(),
                    Block::LooseRoundies { round, empty } => repeat('O')
                        .take(round)
                        .chain(repeat('.').take(empty))
                        .collect(),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    (0..tilted_cols[0].len())
        .map(|row| tilted_cols.iter().map(|col| col[row]).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn load(input: String) -> usize {
    let lines = input.lines().count();

    input
        .lines()
        .enumerate()
        .map(|(index, line)| (lines - index) * line.chars().filter(|c| *c == 'O').count())
        .sum()
}

fn compute(input: String) -> String {
    load(tilt(input)).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tilt() {
        let pre = fs::read_to_string("inputs/example_pre_tilt.txt").unwrap();
        let post = fs::read_to_string("inputs/example_post_tilt.txt").unwrap();
        assert_eq!(post.clone().trim(), tilt(post.clone())); // Tilting is idempotent
        assert_eq!(post.trim(), tilt(pre));
    }

    #[test]
    fn test_load() {
        assert_eq!(load("O".into()), 1);
        assert_eq!(load("OOOO".into()), 4);
    }

    #[test]
    fn test_example() {
        let input = fs::read_to_string("inputs/example_pre_tilt.txt").unwrap();
        assert_eq!("136", compute(input));
    }
}
