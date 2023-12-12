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
                // Could move check closer to where stuff is generated, maybe
                // something like checking if
                // the addition is legal before adding it.
                // The real solution would be to not try to brute force it.
                .filter(|line| requests_start_with(line, &requirements))
                .collect()
        })
        .into_iter()
        .filter(|line| requests_fully_matches(line, &requirements))
        .count()
}

fn requests_fully_matches(line: &[Symbol], requirements: &[usize]) -> bool {
    if let Some(unspent) = requirements_match(line, requirements) {
        return unspent == 0;
    }

    false
}

fn requests_start_with(line: &[Symbol], requirements: &[usize]) -> bool {
    requirements_match(line, requirements).is_some()
}

fn requirements_match(line: &[Symbol], requirements: &[usize]) -> Option<usize> {
    let mut req_index = 0;
    let mut count = 0;

    for sym in line {
        if *sym == Symbol::Broken {
            count += 1;
        } else {
            // Operational
            if count == 0 {
                // Multiple or leading operational ones
                continue;
            }

            // This happens when requirements are greedily consumed
            // There are probably wildcards that aren't being spent as gaps
            if req_index >= requirements.len() {
                return None;
            }

            if count == requirements[req_index] {
                // Passes, move onto the next requirement
                count = 0;

                req_index += 1;
            } else {
                return None;
            }
        }
    }

    // Trailing series of broken
    if count > 0 {
        // Done this way so that it works while checking incompletes
        if req_index >= requirements.len() {
            // Trailing symbols beyond parsing range
            return None;
        } else if count > requirements[req_index] {
            // Already over, abandon
            return None;
        } else if count == requirements[req_index] {
            // This is for when checking full consumption
            req_index += 1;
        }
    }

    Some(requirements.len() - req_index)
}

pub fn compute(input: String) -> String {
    input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            println!("{}", index);
            line_permutations(line)
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn example() {
        let unknowns = fs::read_to_string("inputs/example_unknowns.txt").unwrap();
        assert_eq!("525152", compute(unknowns));
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
            (".??..??...?##. 1,1,3", 16384),
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            ("????.#...#... 4,1,1", 16),
            ("????.######..#####. 1,6,5", 2500),
            ("?###???????? 3,2,1", 506250),
        ] {
            assert_eq!(line_permutations(dbg!(line)), expected);
        }
    }

    #[test]
    fn requirements() {
        for (line, expected) in vec![
            (vec![], Some(3)),
            (vec![Symbol::Broken], Some(2)),
            (vec![Symbol::Broken, Symbol::Operational], Some(2)),
            (
                vec![Symbol::Broken, Symbol::Operational, Symbol::Broken],
                Some(2),
            ),
            (
                vec![
                    Symbol::Broken,
                    Symbol::Operational,
                    Symbol::Broken,
                    Symbol::Broken,
                ],
                Some(1),
            ),
            (
                vec![
                    Symbol::Broken,
                    Symbol::Operational,
                    Symbol::Broken,
                    Symbol::Broken,
                    Symbol::Broken,
                ],
                None,
            ),
            (
                vec![
                    Symbol::Broken,
                    Symbol::Operational,
                    Symbol::Broken,
                    Symbol::Broken,
                    Symbol::Operational,
                    Symbol::Broken,
                ],
                Some(1),
            ),
            (
                vec![
                    Symbol::Broken,
                    Symbol::Operational,
                    Symbol::Broken,
                    Symbol::Broken,
                    Symbol::Operational,
                    Symbol::Broken,
                    Symbol::Broken,
                    Symbol::Broken,
                ],
                Some(0),
            ),
            (
                vec![
                    Symbol::Broken,
                    Symbol::Operational,
                    Symbol::Broken,
                    Symbol::Broken,
                    Symbol::Operational,
                    Symbol::Broken,
                    Symbol::Broken,
                    Symbol::Broken,
                    Symbol::Operational,
                ],
                Some(0),
            ),
        ] {
            assert_eq!(requirements_match(&line, &vec![1, 2, 3]), expected);
        }
    }
}
