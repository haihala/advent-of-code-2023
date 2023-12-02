use day_02::{Cubes, Game};
use std::fs;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("inputs/input1.txt").unwrap();
    println!("{}", compute(input,));
}

fn compute(input: String) -> String {
    input
        .lines()
        .map(|line| {
            let game = Game::from_str(line).unwrap();

            let max_pulls = game.rounds.iter().fold(Cubes::default(), |a, b| a.max(b));
            max_pulls.power()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let example_input = fs::read_to_string("inputs/example1.txt").unwrap();
        assert_eq!(compute(example_input), "2286")
    }
}
