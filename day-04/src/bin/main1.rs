use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", calculate(input));
}

fn calculate(input: String) -> String {
    input
        .lines()
        .map(|line| {
            if line.len() == 0 {
                return 0;
            }

            let no_prefix = line
                .chars()
                .skip_while(|c| *c != ':')
                .skip(2) // Skips the : and the following space
                .collect::<String>();

            let (win_chunk, mine_chunk) = no_prefix.split_once(" | ").unwrap();

            let winners = win_chunk
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<HashSet<_>>();
            let mine = mine_chunk
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<HashSet<_>>();

            let matches = mine.intersection(&winners).count();

            if matches == 0 {
                0 as usize
            } else {
                usize::pow(2, (matches - 1) as u32)
            }
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let input = fs::read_to_string("inputs/example1.txt").unwrap();
        assert_eq!(calculate(input), "13")
    }
}
