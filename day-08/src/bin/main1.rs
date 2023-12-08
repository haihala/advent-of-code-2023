use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", compute(input));
}

fn compute(input: String) -> String {
    let instructions: Vec<bool> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'R' => true,
            'L' => false,
            _ => panic!("Unknown direction {:?}", c),
        })
        .collect();

    let nodes: HashMap<String, (String, String)> = input
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

            let (l, r) = tmp.split_once(", ").unwrap();

            (loc, (l.to_owned(), r.to_owned()))
        })
        .collect();

    let mut instruction_pointer = 0;
    let mut location = "AAA".to_owned();

    while location != "ZZZ".to_owned() {
        let instruction = instructions[instruction_pointer % instructions.len()];
        let node = &nodes[&location];
        location = (if instruction { &node.1 } else { &node.0 }).to_owned();
        instruction_pointer += 1;
    }

    instruction_pointer.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = fs::read_to_string("inputs/example1.txt").unwrap();
        assert_eq!("2", compute(input));
    }

    #[test]
    fn test_example2() {
        let input = fs::read_to_string("inputs/example2.txt").unwrap();
        assert_eq!("6", compute(input));
    }
}
