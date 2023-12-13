use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", compute(input));
}

fn compute(input: String) -> String {
    input
        .split("\n\n")
        .map(|scenario| scenario_score(scenario))
        .sum::<usize>()
        .to_string()
}

fn scenario_score(scenario: &str) -> usize {
    let lines: Vec<_> = scenario.lines().collect();

    // Order matters, patterns can have both
    if let Some(value) = get_col_score(&lines) {
        return value;
    }

    if let Some(value) = get_row_score(&lines) {
        return value;
    }

    dbg!(lines);

    todo!()
}

fn get_col_score(lines: &[&str]) -> Option<usize> {
    let max_col = lines[0].len();

    let cols_equal = |left: usize, right: usize| {
        lines
            .iter()
            .all(|line| line.chars().skip(left).next() == line.chars().skip(right).next())
    };

    let mut potential_cols = vec![];
    for col in 0..(max_col - 1) {
        if cols_equal(col, col + 1) {
            potential_cols.push(col);
        }
    }

    'outer: for col in potential_cols {
        for offset in 1..=(col.min(max_col - col - 2)) {
            if !cols_equal(col - offset, col + offset + 1) {
                continue 'outer;
            }
        }

        // All lines are good
        return Some(col + 1);
    }

    None
}

fn get_row_score(lines: &[&str]) -> Option<usize> {
    let potential_rows: Vec<_> = lines
        .windows(2)
        .enumerate()
        .filter(|(_, win)| {
            let (top, bottom) = (win[0], win[1]);
            top == bottom
        })
        .map(|(index, _)| index)
        .collect();

    'outer: for row in potential_rows {
        for offset in 1..=(row.min(lines.len() - row - 2)) {
            if lines[row - offset] != lines[row + offset + 1] {
                continue 'outer;
            }
        }

        // All lines are good
        // Rows are 1-indexed
        return Some((1 + row) * 100);
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = fs::read_to_string("inputs/example1.txt").unwrap();
        assert_eq!(Some(5), get_col_score(&input.lines().collect::<Vec<_>>()));
        assert_eq!("5", compute(input.clone()));
    }

    #[test]
    fn test_example2() {
        let input = fs::read_to_string("inputs/example2.txt").unwrap();
        assert_eq!(Some(400), get_row_score(&input.lines().collect::<Vec<_>>()));
        assert_eq!("400", compute(input.clone()));
    }

    #[test]
    fn test_example3() {
        let input = fs::read_to_string("inputs/example3.txt").unwrap();
        assert_eq!(Some(300), get_row_score(&input.lines().collect::<Vec<_>>()));
        assert_eq!("300", compute(input.clone()));
    }

    #[test]
    fn test_example4() {
        let input = fs::read_to_string("inputs/example4.txt").unwrap();
        assert_eq!(Some(13), get_col_score(&input.lines().collect::<Vec<_>>()));
        assert_eq!("13", compute(input.clone()));
    }

    #[test]
    fn test_example5() {
        let input = fs::read_to_string("inputs/example5.txt").unwrap();
        assert_eq!(Some(12), get_col_score(&input.lines().collect::<Vec<_>>()));
        assert_eq!("12", compute(input.clone()));
    }

    #[test]
    fn test_example_both() {
        let input = fs::read_to_string("inputs/example_both.txt").unwrap();
        assert_eq!("405", compute(input));
    }
}
