use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", compute(input));
}

fn parse_num_line(input: &mut Vec<&str>) -> isize {
    input
        .pop()
        .unwrap()
        .chars()
        .skip_while(|c| !c.is_digit(10))
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<isize>()
        .unwrap()
}

fn compute(input: String) -> String {
    let mut lines: Vec<&str> = input.lines().collect();

    let distance = parse_num_line(&mut lines);
    let time = parse_num_line(&mut lines);
    (1..time)
        .filter(|i| i * (time - i) > distance)
        .count()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("inputs/example.txt").unwrap();
        assert_eq!("71503", compute(input));
    }
}
