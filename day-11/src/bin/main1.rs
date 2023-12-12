use itertools::Itertools;
use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", compute(input));
}

type Point = (usize, usize);

fn compute(input: String) -> String {
    // Parse as point cloud
    let galaxies: HashSet<Point> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(move |(col, char)| {
                if char == '#' {
                    Some((row as usize, col as usize))
                } else {
                    None
                }
            })
        })
        .collect();

    // Find rows and cols that don't contain galaxies
    let max_row = galaxies.iter().max_by_key(|galaxy| galaxy.0).unwrap();
    let max_col = galaxies.iter().max_by_key(|galaxy| galaxy.1).unwrap();

    let empty_rows: HashSet<_> = (0..max_row.0)
        .filter(|row| !galaxies.iter().any(|galaxy| &galaxy.0 == row))
        .collect();
    let empty_cols: HashSet<_> = (0..max_col.1)
        .filter(|col| !galaxies.iter().any(|galaxy| &galaxy.1 == col))
        .collect();

    // Calculate manhattan distance, adding "duplicated" rows
    galaxies
        .into_iter()
        .combinations(2)
        .map(|pair| {
            let miny = pair.iter().min_by_key(|g| g.0).unwrap().0;
            let minx = pair.iter().min_by_key(|g| g.1).unwrap().1;
            let maxy = pair.iter().max_by_key(|g| g.0).unwrap().0;
            let maxx = pair.iter().max_by_key(|g| g.1).unwrap().1;

            let manhattan_distance = (maxx - minx) + (maxy - miny);

            let extra_rows = (miny..maxy).filter(|row| empty_rows.contains(row)).count();
            let extra_cols = (minx..maxx).filter(|col| empty_cols.contains(col)).count();

            manhattan_distance as usize + extra_rows + extra_cols
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("inputs/example.txt").unwrap();
        assert_eq!("374", compute(input));
    }
}
