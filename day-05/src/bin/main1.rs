use std::{collections::VecDeque, fs, num::ParseIntError, str::FromStr};

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", compute(input));
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Mapping {
    from: isize,
    to: isize,
    length: isize,
}
impl FromStr for Mapping {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .split_whitespace()
            .map(|num| num.parse::<isize>())
            .collect::<Vec<_>>();

        assert_eq!(nums.len(), 3);

        let (to, from, length) = (nums[0].clone()?, nums[1].clone()?, nums[2].clone()?);

        Ok(Self { from, to, length })
    }
}
impl Mapping {
    fn apply(&self, input: usize) -> usize {
        let offset = input as isize - self.from;
        if offset > 0 && offset < self.length {
            (self.to + offset) as usize
        } else {
            input
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Pipeline {
    seeds: Vec<usize>,
    mappings: Vec<Vec<Mapping>>,
}
impl FromStr for Pipeline {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().collect::<VecDeque<_>>();
        let seeds_line = lines.pop_front().unwrap();

        let seeds = seeds_line
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        let mappings = lines.into_iter().fold(vec![], |mut acc, line| {
            if line.is_empty() {
                acc.push(vec![]);
            } else if line.chars().next().unwrap().is_digit(10) {
                acc.last_mut()
                    .unwrap()
                    .push(Mapping::from_str(line).unwrap());
            }

            acc
        });

        Ok(Pipeline { seeds, mappings })
    }
}
impl Pipeline {
    fn process(self) -> Vec<usize> {
        self.mappings
            .into_iter()
            .fold(self.seeds, |seeds, mappings| {
                seeds
                    .into_iter()
                    .map(|seed| {
                        let old = seed;

                        for mapping in &mappings {
                            let new = mapping.apply(old);
                            if new != old {
                                return new;
                            }
                        }

                        old
                    })
                    .collect()
            })
    }
}

fn compute(input: String) -> String {
    let pipe = Pipeline::from_str(&input).unwrap();

    pipe.process().into_iter().min().unwrap().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("inputs/example.txt").unwrap();
        assert_eq!(compute(input), "35");
    }
}
