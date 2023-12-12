use itertools::Itertools;
use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", compute(input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Symbol {
    Operational,
    Broken,
    Unknown,
}
impl From<char> for Symbol {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Broken,
            '?' => Self::Unknown,

            _ => panic!("Unknown char '{:?}'", value),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Chunk(Vec<Symbol>);
impl Chunk {
    fn split_points(&self) -> Vec<usize> {
        // Split points require broken springs on both sides
        self.0
            .iter()
            .enumerate()
            .tuple_windows()
            .filter_map(|(_, (index, elem), _)| {
                if *elem == Symbol::Unknown {
                    Some(index)
                } else {
                    None
                }
            })
            .collect()
    }

    fn split_at(&self, index: usize) -> (Chunk, Chunk) {
        let (first, second) = self.0.split_at(index);
        (
            Chunk(first.to_vec()),
            Chunk(second.into_iter().skip(1).cloned().collect()),
        )
    }

    // Assumes a single continuous block
    fn permutations(&self, goal: usize) -> usize {
        let broken = self.0.iter().filter(|sym| **sym == Symbol::Broken).count();
        if goal < broken {
            return 0;
        }
        if goal == broken {
            // This is for goal 0 cases
            return 1;
        }

        let broken_indices = self
            .0
            .iter()
            .enumerate()
            .filter_map(|(index, sym)| {
                if *sym == Symbol::Broken {
                    Some(index)
                } else {
                    None
                }
            })
            .collect_vec();

        if let (Some(first), Some(last)) =
            (broken_indices.iter().min(), broken_indices.iter().max())
        {
            // Have to fill the gap first
            let gap = last - first + 1;
            let to_spend = goal - gap;
            let after_last = self.0.len() - last - 1;
            let max_before_gap = first.min(&to_spend);

            if max_before_gap + after_last < to_spend {
                // There is not enough space to get that many
                0
            } else {
                let min_after_last = to_spend - max_before_gap;
                after_last - min_after_last + 1
            }
        } else {
            // All question marks
            match self.0.len().cmp(&goal) {
                std::cmp::Ordering::Less => 0, // Can't form
                std::cmp::Ordering::Equal => 1,
                std::cmp::Ordering::Greater => self.0.len() - goal + 1,
            }
        }
    }
}
impl From<char> for Chunk {
    fn from(value: char) -> Self {
        let sym = Symbol::from(value);

        Chunk(if sym == Symbol::Operational {
            vec![]
        } else {
            vec![sym]
        })
    }
}

fn str_to_chunkvec(input: &str) -> Vec<Chunk> {
    let mut chunks: Vec<_> = input.chars().fold(vec![], |mut acc, c| {
        let Some(prev) = acc.last_mut() else {
            // First
            return vec![Chunk::from(c)];
        };

        match Symbol::from(c) {
            Symbol::Operational => {
                if *prev != Chunk::default() {
                    acc.push(Chunk::default());
                }
            }
            other => {
                prev.0.push(other);
            }
        };

        acc
    });

    // There is probably a clean declarative solution for this
    if chunks.last().unwrap() == &Chunk::default() {
        chunks.pop();
    }

    chunks
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Tree {
    chunk: Chunk,
    permutations: HashMap<usize, usize>,
    children: Vec<Tree>,
}
impl Tree {
    fn from_chunk(chunk: Chunk) -> Self {
        let sp = chunk.split_points();
        let children: Vec<Tree> = sp
            .into_iter()
            .flat_map(|sp| {
                let (l, r) = chunk.split_at(sp);

                vec![Self::from_chunk(l), Self::from_chunk(r)]
            })
            .collect();

        let permutations = (0..=chunk.0.len())
            .filter_map(|goal| {
                let biggest = vec![chunk.to_owned().permutations(goal)]
                    .into_iter()
                    .chain(
                        children
                            .iter()
                            .map(|t| t.permutations.get(&goal).unwrap_or(&0).to_owned()),
                    )
                    .max()
                    .unwrap();
                if biggest == 0 {
                    None
                } else {
                    Some((goal, biggest))
                }
            })
            .collect();

        Self {
            children,
            chunk,
            permutations,
        }
    }

    fn max_permutation(&self, splits: usize) -> HashMap<usize, usize> {
        if splits == 0 {
            return self.permutations.clone();
        }

        self.children
            .iter()
            .map(|child| child.max_permutation(splits - 1))
            .fold(HashMap::new(), |mut acc, new| {
                for (key, val) in new {
                    acc.insert(key, val.max(acc.get(&key).unwrap_or(&0).to_owned()));
                }

                acc
            })
    }

    fn max_splits(&self) -> usize {
        self.children
            .iter()
            .map(|child| child.max_splits() + 1)
            .max()
            .unwrap_or(0) // No children => 0
    }
}

fn line_permutations(input: &str) -> usize {
    let (map, digits) = dbg!(input).split_once(" ").unwrap();

    let checks: Vec<usize> = digits
        .split(",")
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    str_to_chunkvec(map)
        .into_iter()
        .map(|chunk| Tree::from_chunk(chunk))
        .map(|tree| {
            // Max permutations(mapping from goal to permutations) for each split
            (0..=tree.max_splits())
                .map(|splits| tree.max_permutation(splits))
                .enumerate()
                .collect_vec()
        })
        // Above this, we have 1d iterator over the line, inside which we have something that maps
        // from splits to max permutations
        .fold(
            vec![],
            |acc: Vec<Vec<(usize, HashMap<usize, usize>)>>, splits_to_perms| {
                if acc.is_empty() {
                    return vec![splits_to_perms];
                }

                // We'd like to have several paths through the line
                acc.into_iter()
                    .cartesian_product(splits_to_perms.into_iter())
                    .map(|(old, new)| {
                        old.into_iter()
                            .map(|(splits, mapping)| {
                                (splits + new.0, {
                                    new.1
                                        .clone()
                                        .into_iter()
                                        .cartesian_product(
                                            // Programming
                                            mapping.into_iter().collect_vec().into_iter(),
                                        )
                                        // Sum keys, multiply values
                                        // Key = How many things are here
                                        // Value = permutations to get that many things
                                        .map(|((n_key, n_val), (o_key, o_val))| {
                                            (n_key + o_key, n_val * o_val)
                                        })
                                        .collect()
                                })
                            })
                            .collect_vec()
                    })
                    .collect_vec()
            },
        )
        .into_iter()
        .map(|path| {
            path.into_iter()
                .flat_map(|(_, goal_to_max_perms)| {
                    goal_to_max_perms
                        .into_iter()
                        .map(move |(goal, perms)| (goal, perms))
                })
                .fold((vec![], 0), |mut acc, step| {
                    (
                        {
                            acc.0.push(step.0);
                            acc.0
                        },
                        acc.1 + step.1,
                    )
                })
        })
        // Now we have a vec of paths through the thing
        .filter(|(splits, _)| *splits == checks)
        .map(|(_, max_permutations)| max_permutations)
        .max()
        .unwrap()
}

fn compute(input: String) -> String {
    input
        .lines()
        .map(|line| line_permutations(line))
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let unknowns = fs::read_to_string("inputs/example_unknowns.txt").unwrap();
        assert_eq!("21", compute(unknowns));
    }

    #[test]
    fn no_unknowns() {
        let no_unknowns = fs::read_to_string("inputs/example_all_knowns.txt").unwrap();
        assert_eq!(
            // One per line, lines sum up
            no_unknowns.lines().count().to_string(),
            compute(no_unknowns)
        );
    }

    #[test]
    fn example_lines() {
        for (line, expected) in vec![
            ("???.### 1,1,3", 1),
            (".??..??...?##. 1,1,3", 4),
            ("?###???????? 3,2,1", 10),
        ] {
            assert_eq!(line_permutations(line), expected);
        }
    }

    #[test]
    fn test_splitting1() {
        let cv = str_to_chunkvec("#####");
        assert_eq!(cv.len(), 1);
        assert_eq!(cv.first().unwrap().split_points(), vec![]);
    }

    #[test]
    fn test_splitting2() {
        let cv = str_to_chunkvec("##?##");
        assert_eq!(cv.len(), 1);
        assert_eq!(cv.first().unwrap().split_points(), vec![2]);
    }

    #[test]
    fn test_splitting3() {
        let cv = str_to_chunkvec("##?##");
        assert_eq!(cv.len(), 1);
        assert_eq!(
            cv.first().unwrap().split_at(2),
            (
                str_to_chunkvec("##")[0].clone(),
                str_to_chunkvec("##")[0].clone()
            )
        );
    }

    #[test]
    fn test_permutations() {
        for (line, goal, expected) in vec![
            ("#", 1, 1),
            ("?", 1, 1),
            ("?#", 2, 1),
            ("?#?", 2, 2),
            ("??#?", 2, 2),
            ("???#?", 2, 2),
            ("???#?", 3, 2),
        ] {
            assert_eq!(
                str_to_chunkvec(line)[0].clone().permutations(goal),
                expected
            );
        }
    }
}
