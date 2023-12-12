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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Requirement {
    len: usize,
    fulfilling_indices: Vec<usize>,
    fulfilled_by: Vec<usize>,
    score: usize,
}
impl Requirement {
    fn fullfill_with(&mut self, fulfilling_indices: Vec<usize>, score: usize) {
        self.fulfilled_by = fulfilling_indices;
        self.fulfilling_indices = vec![];
        self.score = score;
    }
}

fn line_permutations(input: &str) -> usize {
    let (map, digits) = dbg!(input).split_once(" ").unwrap();

    let mut requirements: Vec<Requirement> = digits
        .split(",")
        .map(|num| Requirement {
            len: num.parse::<usize>().unwrap(),
            ..Requirement::default()
        })
        .collect();

    let bundles = get_bundles(map);

    // Eliminate ones where only one variation is legal
    let borks = bundles
        .iter()
        .enumerate()
        .filter(|(_, syms)| syms.iter().all(|sym| *sym == Symbol::Broken))
        .collect_vec();

    // The thingy at index is all bork
    for req in &mut requirements {
        for (index, _) in borks.iter().filter(|(_, syms)| syms.len() == req.len) {
            req.fulfilling_indices.push(*index)
        }
    }

    // The borks gotta go somewhere
    // If a bork only goes into one place, that is where it shall go
    for (index, _) in &borks {
        let mut reqs = requirements
            .iter_mut()
            .filter(|req| req.fulfilling_indices.contains(index));

        if let Some(first) = reqs.next() {
            if reqs.next().is_none() {
                // This one only goes here
                first.fullfill_with(vec![*index], 1);

                // Since we found a place for this one, remove it from the others
                for req in &mut requirements {
                    req.fulfilling_indices.retain(|bundle| bundle != index)
                }
            }
        }
    }

    // This goes for until we can't deduce any more
    loop {
        let mut index = 0;

        // This goes for until we find one to change
        let output = loop {
            if index == requirements.len() - 1 {
                break None;
            }

            let left = requirements.get(index).unwrap();
            let right = requirements.get(index + 1).unwrap();

            let left_fulfilled = left.score > 0;
            let right_fulfilled = right.score > 0;

            if left_fulfilled == right_fulfilled {
                // Either both are fulfilled or neither one is
                // Nothing to do here
                index += 1;
                continue;
            }

            if left_fulfilled {
                // Right one isn't
                let fulfilling_bundle = left.fulfilled_by[0];
                let potential = fulfilling_bundle + 1;
                if right.fulfilling_indices.contains(&potential) {
                    break Some((index + 1, potential));
                }
            }

            if right_fulfilled {
                // Left one isn't
                let fulfilling_bundle = right.fulfilled_by[0];
                let potential = fulfilling_bundle - 1;
                if left.fulfilling_indices.contains(&potential) {
                    break Some((index, potential));
                }
            }

            index += 1;
        };

        if let Some((target_req, target_bork)) = output {
            requirements
                .get_mut(target_req)
                .unwrap()
                .fullfill_with(vec![target_bork], 1);
        } else {
            break;
        }
    }

    dbg!(&requirements);

    requirements.into_iter().map(|req| req.score).product()
}

fn get_bundles(map: &str) -> Vec<Vec<Symbol>> {
    let tokens = map.chars().map(|char| Symbol::from(char)).collect_vec();
    let mut new_bundle = true;

    tokens
        .into_iter()
        .skip_while(|token| token == &Symbol::Operational)
        .fold(vec![], |mut acc, token| {
            match (token, new_bundle) {
                (Symbol::Operational, _) => {
                    new_bundle = true;
                }
                (sym, true) => {
                    acc.push(vec![sym]);
                    new_bundle = false;
                }
                (sym, false) => {
                    acc.last_mut().unwrap().push(sym);
                }
            };
            acc
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
