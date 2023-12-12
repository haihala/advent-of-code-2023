use itertools::Itertools;
use std::iter;

pub fn line_permutations(input: &str) -> usize {
    let (map, digits) = input.split_once(" ").unwrap();

    let requirements: Vec<usize> = iter::repeat(digits)
        .take(5)
        .join(",")
        .split(",")
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let input = iter::repeat(map).take(5).join("?");

    recurse(requirements, input)
}

#[memoize::memoize]
fn recurse(req: Vec<usize>, input: String) -> usize {
    if req.is_empty() {
        return !input.contains('#') as usize;
    } else if input.is_empty() {
        return 0; // Still requests to go and no input
    }

    if input.len() < req.iter().sum::<usize>() + req.len() - 1 {
        return 0;
    }

    let mut chars = input.chars();

    match chars.next().unwrap() {
        '#' => hash_recurse(req, input),
        '.' => recurse(req, chars.collect()),
        '?' => recurse(req.clone(), chars.collect()) + hash_recurse(req, input),
        _ => panic!("Invalid character"),
    }
}

fn hash_recurse(req: Vec<usize>, input: String) -> usize {
    let first_req = req.iter().next().unwrap();
    let chunk = input.chars().take(*first_req).collect_vec();

    if !chunk.contains(&'.') && input.chars().skip(*first_req).next() != Some('#') {
        let start_index = *first_req + 1;

        recurse(
            req.into_iter().skip(1).collect(),
            input.chars().skip(start_index).collect(),
        )
    } else {
        0
    }
}

pub fn compute(input: String) -> String {
    input
        .lines()
        .map(|line| line_permutations(line))
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
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            ("????.#...#... 4,1,1", 16), // Slow
            ("????.######..#####. 1,6,5", 2500),
            ("?###???????? 3,2,1", 506250), // Mega slow
        ] {
            assert_eq!(line_permutations(dbg!(line)), expected);
        }
    }

    #[test]
    fn test_recurse() {
        for (input, requests, expected) in vec![
            ("".into(), vec![], 1),
            (".".into(), vec![], 1),
            ("?".into(), vec![], 1),
            ("#".into(), vec![], 0),
            ("#".into(), vec![1], 1),
            ("##".into(), vec![2], 1),
            ("#?".into(), vec![2], 1),
            ("#?#".into(), vec![3], 1),
            ("#?#".into(), vec![2], 0),
        ] {
            assert_eq!(recurse(dbg!(requests), dbg!(input)), expected)
        }
    }
}
