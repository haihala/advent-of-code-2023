use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", compute(input));
}

fn compute(input: String) -> String {
    input
        .lines()
        .map(|line| {
            let mut first = None;
            let mut last = None;

            for char in line.chars() {
                if !char.is_digit(10) {
                    continue;
                }

                last = Some(char);
                if first.is_none() {
                    first = Some(char)
                }
            }

            [first.unwrap(), last.unwrap()]
                .into_iter()
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let example_input = fs::read_to_string("inputs/example1.txt").unwrap();
        assert_eq!(compute(example_input), "142")
    }
}
