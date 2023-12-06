use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", compute(input));
}

struct Goal {
    time: isize,
    distance: isize,
}

fn parse_num_line(input: &mut Vec<&str>) -> Vec<isize> {
    input
        .pop()
        .unwrap()
        .chars()
        .skip_while(|c| !c.is_digit(10))
        .collect::<String>()
        .split_whitespace()
        .map(|val| val.parse::<isize>().unwrap())
        .collect()
}

fn compute(input: String) -> String {
    let mut lines: Vec<&str> = input.lines().collect();

    let goals: Vec<Goal> = parse_num_line(&mut lines)
        .into_iter()
        .zip(parse_num_line(&mut lines).into_iter())
        .map(|(distance, time)| Goal { distance, time })
        .collect();

    // Naive solution first
    goals
        .into_iter()
        .map(|goal| {
            (1..(goal.time))
                .filter(|i| i * (goal.time - i) > goal.distance)
                .count()
        })
        .reduce(|a, b| a * b)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("inputs/example.txt").unwrap();
        assert_eq!("288", compute(input));
    }
}
