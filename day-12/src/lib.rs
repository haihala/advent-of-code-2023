use itertools::Itertools;
use std::iter;

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

pub fn line_permutations(input: &str) -> usize {
    let (map, digits) = input.split_once(" ").unwrap();

    let requirements: Vec<usize> = iter::repeat(digits)
        .take(5)
        .join(",")
        .split(",")
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    iter::repeat(map)
        .take(5)
        .join("?")
        .chars()
        .map(|char| Symbol::from(char))
        .fold(vec![vec![]], |branches, sym| {
            branches
                .into_iter()
                .flat_map(|mut inner| match sym {
                    Symbol::Unknown => {
                        let mut fork = inner.clone();

                        if let Some(last) = inner.last_mut() {
                            *last += 1;

                            if *last > 1 {
                                // Was not 0, meaning add a new zero
                                fork.push(0);
                            }
                        } else {
                            inner.push(1);
                            fork.push(0);
                        }

                        vec![inner, fork]
                    }
                    Symbol::Broken => {
                        if let Some(last) = inner.last_mut() {
                            *last += 1;
                        } else {
                            inner.push(1);
                        }

                        vec![inner]
                    }
                    Symbol::Operational => {
                        if inner.last_mut().is_none() || inner.last().unwrap() != &0 {
                            inner.push(0);
                        }
                        vec![inner]
                    }
                })
                .filter(|line| requirements_match(line, &requirements))
                .collect()
        })
        .into_iter()
        .filter(|line| line.into_iter().filter(|n| **n != 0).cloned().collect_vec() == requirements)
        .count()
}

fn requirements_match(line: &[usize], requirements: &[usize]) -> bool {
    for (index, bork) in line.iter().filter(|n| **n != 0).enumerate() {
        if index >= requirements.len() {
            // Too many splits, not legal
            return false;
        }

        let req = requirements[index];

        if bork > &req {
            return false;
        }
    }

    true
}

pub fn compute(input: String) -> String {
    input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            println!("{}", index);
            dbg!(line_permutations(dbg!(line)))
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    // #[test]
    // fn example() {
    //     let unknowns = fs::read_to_string("inputs/example_unknowns.txt").unwrap();
    //     assert_eq!("525152", compute(unknowns));
    // }

    #[test]
    fn no_unknowns() {
        for line in fs::read_to_string("inputs/example_all_knowns.txt")
            .unwrap()
            .lines()
        {
            assert_eq!(line_permutations(dbg!(line)), 1);
        }
    }

    #[test]
    fn example_lines() {
        for (line, expected) in vec![
            ("???.### 1,1,3", 1),
            (".??..??...?##. 1,1,3", 16384), // Slow
                                             // ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
                                             // ("????.#...#... 4,1,1", 16),    // Slow
                                             // ("????.######..#####. 1,6,5", 2500),
                                             // ("?###???????? 3,2,1", 506250), // Mega slow
        ] {
            assert_eq!(line_permutations(dbg!(line)), expected);
        }
    }

    #[test]
    fn requirements() {
        for (line, expected) in vec![
            (vec![], true),
            (vec![1], true),
            (vec![1, 1], true),
            (vec![1, 2], true),
            (vec![1, 3], false),
            (vec![1, 2, 1], true),
            (vec![1, 2, 3], true),
        ] {
            assert_eq!(requirements_match(dbg!(&line), &vec![1, 2, 3]), expected);
        }
    }
}
