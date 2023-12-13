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

fn col_diff(lines: &[&str], left: usize, right: usize) -> usize {
    lines
        .iter()
        .filter(|line| line.chars().skip(left).next() != line.chars().skip(right).next())
        .count()
}

fn get_col_score(lines: &[&str]) -> Option<usize> {
    let max_col = lines[0].len();

    let potential_cols: Vec<_> = (0..(max_col - 1))
        .filter_map(|col| {
            let cd = col_diff(lines, col, col + 1);
            if cd > 1 {
                None
            } else {
                Some((col, cd))
            }
        })
        .collect();

    for (col, diff) in potential_cols {
        let sum: usize = (1..=(col.min(max_col - col - 2)))
            .map(|offset| col_diff(lines, col - offset, col + offset + 1))
            .sum();

        if diff + sum == 1 {
            // Exactly one smudge
            return Some(col + 1);
        }
    }

    None
}

fn row_diff(line1: &str, line2: &str) -> usize {
    line1
        .chars()
        .zip(line2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

fn get_row_score(lines: &[&str]) -> Option<usize> {
    let potential_rows: Vec<_> = lines
        .windows(2)
        .enumerate()
        .filter_map(|(index, win)| {
            let (top, bottom) = (win[0], win[1]);
            let rd = row_diff(top, bottom);

            if rd > 1 {
                None
            } else {
                Some((index, rd))
            }
        })
        .collect();

    for (row, diff) in potential_rows {
        let sum: usize = (1..=(row.min(lines.len() - row - 2)))
            .map(|offset| row_diff(lines[row - offset], lines[row + offset + 1]))
            .sum();

        if diff + sum == 1 {
            // Exactly one smudge
            return Some((1 + row) * 100);
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = fs::read_to_string("inputs/example1.txt").unwrap();
        assert_eq!(Some(300), get_row_score(&input.lines().collect::<Vec<_>>()));
        assert_eq!("300", compute(input.clone()));
    }

    #[test]
    fn test_example2() {
        let input = fs::read_to_string("inputs/example2.txt").unwrap();
        assert_eq!(Some(100), get_row_score(&input.lines().collect::<Vec<_>>()));
        assert_eq!("100", compute(input.clone()));
    }

    #[test]
    fn test_example_both() {
        let input = fs::read_to_string("inputs/example_both.txt").unwrap();
        assert_eq!("400", compute(input));
    }

    #[test]
    fn test_row_diff() {
        assert_eq!(
            row_diff("lorem ipsum solem dolor", "lorem ipsum solem dolor"),
            0
        );
        assert_eq!(
            row_diff("lorem ipsum solem doloi", "lorem ipsum solem dolor"),
            1
        );
        assert_eq!(
            row_diff("lorem ipsum solem #####", "lorem ipsum solem dolor"),
            5
        );
    }

    #[test]
    fn test_col_diff() {
        let input = fs::read_to_string("inputs/example1.txt").unwrap();

        for (col1, col2, diff) in vec![
            (4, 5, 0),
            (3, 6, 0),
            (2, 7, 0),
            (1, 8, 0),
            // Self should be 0
            (1, 1, 0),
            (2, 2, 0),
            (3, 3, 0),
            (4, 4, 0),
            (5, 5, 0),
            (6, 6, 0),
            (7, 7, 0),
            (8, 8, 0),
            // Some random lines
            (0, 1, 2),
            (1, 0, 2),
            (1, 2, 7),
        ] {
            assert_eq!(
                col_diff(&(input.lines().collect::<Vec<_>>()), col1, col2),
                diff
            );
        }
    }
}
