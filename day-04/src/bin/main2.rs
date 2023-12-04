use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", calculate(input));
}

fn calculate(input: String) -> String {
    let matches = input
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

            mine.intersection(&winners).count()
        })
        .collect::<Vec<usize>>();

    let mut cards = 0;
    let mut upcoming: VecDeque<usize> = VecDeque::new();

    for points in matches {
        let instances = upcoming.pop_front().unwrap_or_default() + 1;
        cards += instances;

        for i in 0..points {
            if upcoming.len() <= i {
                upcoming.push_back(instances);
            } else {
                let copies = upcoming.get_mut(i).unwrap();
                *copies += instances;
            }
        }
    }

    cards.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let input = fs::read_to_string("inputs/example1.txt").unwrap();
        assert_eq!(calculate(input), "30")
    }
}
