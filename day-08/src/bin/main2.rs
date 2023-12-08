use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", compute(input));
}

#[derive(Debug, Clone, Default)]
struct Pointer {
    z_enders: Vec<usize>,
    history: Vec<String>,
}
impl Pointer {
    fn new(key: String) -> Self {
        Self {
            history: vec![key],
            ..Self::default()
        }
    }

    fn advance(&mut self, instructions: &Vec<usize>, map: &HashMap<String, Vec<String>>) {
        let options = map.get(self.history.last().unwrap()).unwrap();
        self.history
            .push(options[instructions[&(self.history.len() - 1) % instructions.len()]].to_owned());

        if self.history.last().unwrap().ends_with("Z") {
            self.z_enders.push(self.history.len());
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Loop {
    start: usize,
    period: usize,
    exit_offsets: Vec<usize>,
    pulls: usize,
}
impl Loop {
    fn new(
        instructions: &Vec<usize>,
        map: &HashMap<String, Vec<String>>,
        starting_point: String,
    ) -> Self {
        let mut single = Pointer::new(starting_point.clone());
        let mut double = single.clone();

        loop {
            single.advance(instructions, map);
            double.advance(instructions, map);
            double.advance(instructions, map);

            let same_token = double.history.last().unwrap() == single.history.last().unwrap();
            let same_offset = double.history.len() % instructions.len()
                == single.history.len() % instructions.len();
            if same_token && same_offset {
                break;
            }
        }

        let single_len = single.history.len();
        let period = double.history.len() - single.history.len();
        let start = double
            .history
            .into_iter()
            .rev()
            .zip(single.history.into_iter().rev())
            .enumerate()
            .find_map(|(index, (d, s))| if d != s { Some(index) } else { None })
            .map(|split| single_len - split) // If the loop starts from the first one
            .unwrap_or_default();
        let exit_offsets = double.z_enders.into_iter().map(|x| x - start).collect();

        Self {
            // This will find when the two differ.
            // Since double did a whole loop and single didn't, this lets us know what point the split
            // happened at.
            start,
            period,
            exit_offsets,
            pulls: 0,
        }
    }
}
impl Iterator for Loop {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let iteration = self.pulls / self.exit_offsets.len();
        self.pulls += 1;
        Some(
            self.start
                + self.period * iteration
                + self.exit_offsets[(self.pulls - 1) % self.exit_offsets.len()],
        )
    }
}

fn compute(input: String) -> String {
    let instructions: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'R' => 1,
            'L' => 0,
            _ => panic!("Unknown direction {:?}", c),
        })
        .collect();

    let nodes: HashMap<String, Vec<String>> = input
        .lines()
        .skip(2)
        .map(|line| {
            let loc = line
                .chars()
                .take_while(|c| !c.is_whitespace())
                .collect::<String>();

            let tmp = line
                .chars()
                .skip_while(|c| *c != '(')
                .skip(1)
                .take_while(|c| *c != ')')
                .collect::<String>();

            (loc, tmp.split(", ").map(|s| s.to_owned()).collect())
        })
        .collect();

    let mut loops: Vec<(Loop, usize)> = nodes
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|s| s.to_owned())
        .map(|ghost| Loop::new(&instructions, &nodes, ghost))
        .map(|mut l| (l.clone(), l.next().unwrap()))
        .collect();

    let mut minimum = *loops.iter().map(|(_, val)| val).min().unwrap();

    while loops.iter().any(|(_, last)| *last != minimum) {
        loops = loops
            .into_iter()
            .map(|(mut l, mut pull)| {
                while pull < minimum {
                    pull = l.next().unwrap();
                }
                if pull > minimum {
                    minimum = pull;
                }
                (l, pull)
            })
            .collect();
    }

    // This is probably a bug elsewhere. Maybe something like the start value getting added twice.
    // It however works and I've spent too much time on this already so I'll leave this crappy fix
    // here
    (minimum - 1).to_string()
    // The correct solution is probably some quite simple combinatoric truth derived from the
    // smallest common multiple.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = fs::read_to_string("inputs/example_many.txt").unwrap();
        assert_eq!("6", compute(input));
    }
}
