use std::{collections::VecDeque, fs, num::ParseIntError, ops::Range, str::FromStr};

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", compute(input));
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Mapping {
    range: Range<isize>,
    shift: isize,
}
impl FromStr for Mapping {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .split_whitespace()
            .map(|num| num.parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(nums.len(), 3);
        let (to, from, length) = (nums[0], nums[1], nums[2]);

        Ok(Self {
            range: from..(from + length),
            shift: to - from,
        })
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct RangeMapping {
    mappings: Vec<Mapping>,
}
impl RangeMapping {
    fn from_mapping_vec(mut input: Vec<Mapping>) -> Self {
        input.sort_by(|a, b| a.range.start.cmp(&b.range.start));

        Self { mappings: input }
    }

    fn apply(&self, seed_group: Range<isize>) -> Vec<Range<isize>> {
        let mut out = vec![];
        let mut marker = seed_group.start;

        for mapping in &self.mappings {
            if marker > mapping.range.end {
                continue;
            }
            let end_marker = seed_group.end.min(mapping.range.end);
            if mapping.range.contains(&marker) {
                out.push((marker + mapping.shift)..(end_marker + mapping.shift));
                marker = end_marker;
            } else if mapping.range.start > marker {
                let mid_marker = mapping.range.start.min(seed_group.end);
                out.push(marker..mid_marker);

                if mapping.range.start < seed_group.end {
                    out.push((mapping.range.start + mapping.shift)..(end_marker + mapping.shift));
                    marker = end_marker;
                }
            }
        }

        if out.is_empty() {
            // None of the mappings reach
            vec![seed_group]
        } else {
            out
        }
    }
}

fn parse_seed_line(input: &str) -> Vec<Range<isize>> {
    let seed_tokens = input
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    seed_tokens
        .chunks(2)
        .map(|elems| {
            let (base, size) = (elems[0], elems[1]);
            base..(base + size)
        })
        .collect()
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Pipeline {
    seeds: Vec<Range<isize>>,
    mappings: Vec<RangeMapping>,
}
impl FromStr for Pipeline {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().collect::<VecDeque<_>>();
        let seeds_line = lines.pop_front().unwrap();
        let seeds = parse_seed_line(seeds_line);

        let mappings = lines
            .into_iter()
            .fold(vec![], |mut acc, line| {
                if line.is_empty() {
                    acc.push(vec![]);
                } else if line.chars().next().unwrap().is_digit(10) {
                    acc.last_mut()
                        .unwrap()
                        .push(Mapping::from_str(line).unwrap());
                }

                acc
            })
            .into_iter()
            .map(RangeMapping::from_mapping_vec)
            .collect();

        Ok(Pipeline { seeds, mappings })
    }
}
impl Pipeline {
    fn process(mut self) -> Vec<isize> {
        for round in self.mappings.into_iter() {
            self.seeds = Self::dedup_ranges(
                self.seeds
                    .into_iter()
                    .flat_map(|seed_group| round.apply(seed_group))
                    .collect(),
            );
        }

        self.seeds.into_iter().flatten().collect()
    }

    fn dedup_ranges(mut input: Vec<Range<isize>>) -> Vec<Range<isize>> {
        input.sort_by(|a, b| a.start.cmp(&b.start));
        let mut coll = vec![];

        for i in input.into_iter() {
            if coll.is_empty() {
                coll.push(i);
                continue;
            }

            let current_end = coll.last().unwrap().end;
            if current_end > i.start {
                let last = coll.last_mut().unwrap();
                last.end = i.end.max(current_end);
            } else {
                coll.push(i);
            }
        }

        coll
    }
}

fn compute(input: String) -> String {
    let pipe = Pipeline::from_str(&input).unwrap();
    pipe.process().into_iter().min().unwrap().to_string()
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("inputs/example.txt").unwrap();
        assert_eq!(compute(input), "46");
    }

    #[test]
    fn test_seed_parsing() {
        let seed = parse_seed_line("seeds: 79 14 55 13");

        assert_eq!(
            seed.into_iter().collect::<HashSet<_>>(),
            vec![79..93, 55..68].into_iter().collect()
        );
    }

    #[test]
    fn test_for_overlaping_ranges() {
        let input = fs::read_to_string("inputs/input.txt").unwrap();
        let pipe = Pipeline::from_str(&input).unwrap();

        pipe.mappings.into_iter().for_each(|layer| {
            let mut end = -1;
            for mapping in &layer.mappings {
                assert!(!mapping.range.contains(&end));
                end = mapping.range.end - 1; // Gives the non-inclusive end
            }
        });
    }
}
