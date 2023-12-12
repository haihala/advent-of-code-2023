use itertools::Itertools;
use std::fs;

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

fn line_permutations(input: &str) -> usize {
    let (map, digits) = input.split_once(" ").unwrap();

    let requirements: Vec<usize> = digits
        .split(",")
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let possible_lines = brute_force(map);

    possible_lines
        .into_iter()
        .filter(|line| {
            // Form the streaks or broken
            line.into_iter()
                .fold(vec![0], |mut acc, sym| {
                    if *sym == Symbol::Broken {
                        let last = acc.last_mut().unwrap();
                        *last += 1;
                    } else {
                        // Operational
                        let last = acc.last().unwrap();
                        if *last != 0 {
                            acc.push(0);
                        }
                    }

                    acc
                })
                .into_iter()
                // If there are trailing intact ones, they add a zero to the end
                .filter(|e| *e != 0)
                .collect_vec()
                == requirements
        })
        .count()
}

fn brute_force(map: &str) -> Vec<Vec<Symbol>> {
    // Top level is bundle, second level is variation, third level is symbols
    map.chars()
        .map(|char| Symbol::from(char))
        .fold(vec![vec![]], |branches, sym| {
            branches
                .into_iter()
                .flat_map(|mut inner| {
                    if sym == Symbol::Unknown {
                        let mut out1 = inner.clone();
                        out1.push(Symbol::Operational);
                        let mut out2 = inner.clone();
                        out2.push(Symbol::Broken);

                        vec![out1, out2]
                    } else {
                        inner.push(sym);
                        vec![inner]
                    }
                })
                .collect()
        })
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
}
