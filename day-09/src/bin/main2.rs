use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", compute(input));
}

fn compute(input: String) -> String {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|new| {
            let mut stack = vec![new.clone()];
            while stack.last().unwrap().iter().any(|num| *num != 0) {
                // All of them are not zeros, make a new layer
                let prev = stack.last().unwrap().clone();
                assert!(prev.len() > 1);
                stack.push(prev.windows(2).fold(vec![], |mut acc, window| {
                    acc.push(window[1] - window[0]);
                    acc
                }))
            }

            stack
                .into_iter()
                .rev()
                .fold(0, |curr, layer| layer.first().unwrap() - curr)
        })
        .sum::<isize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("inputs/example.txt").unwrap();
        assert_eq!("2", compute(input));
    }
}
