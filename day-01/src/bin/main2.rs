use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", compute(input));
}

fn compute(input: String) -> String {
    input.lines().map(line_value).sum::<u32>().to_string()
}

fn line_value(input: &str) -> u32 {
    let subs: HashMap<String, char> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .into_iter()
    .enumerate()
    .map(|(index, val)| {
        (
            val.to_string(),
            char::from_digit(index as u32 + 1, 10).unwrap(),
        )
    })
    .chain("123456789".chars().map(|char| (char.to_string(), char)))
    .collect();

    let mut first = None;
    let mut last = None;

    let mut strinput = input.to_string();
    'outer: while first.is_none() {
        for (pattern, value) in &subs {
            if strinput.starts_with(pattern) {
                first = Some(value);
                break 'outer;
            }
        }

        // Didn't find
        strinput.remove(0);
    }

    'outer: while last.is_none() {
        for (pattern, value) in &subs {
            if strinput.ends_with(pattern) {
                last = Some(value);
                break 'outer;
            }
        }

        // Didn't find
        strinput.pop();
    }

    [first.unwrap(), last.unwrap()]
        .into_iter()
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let example_input = fs::read_to_string("inputs/example2.txt").unwrap();
        assert_eq!(compute(example_input), "281");
    }
}
