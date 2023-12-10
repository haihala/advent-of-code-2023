use std::{collections::HashMap, fs, str::FromStr};

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", compute(input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}
impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(());
        };

        Ok(match s {
            "|" => Tile::NS,
            "-" => Tile::EW,
            "L" => Tile::NE,
            "J" => Tile::NW,
            "7" => Tile::SW,
            "F" => Tile::SE,
            "." => Tile::Ground,
            "S" => Tile::Start,
            _ => return Err(()),
        })
    }
}
impl Tile {
    fn connections(&self, coord: (usize, usize)) -> Vec<(usize, usize)> {
        let up = (coord.0.max(1) - 1, coord.1);
        let down = (coord.0 + 1, coord.1);
        let left = (coord.0, coord.1.max(1) - 1);
        let right = (coord.0, coord.1 + 1);

        match self {
            Tile::NS => vec![up, down],
            Tile::EW => vec![left, right],
            Tile::NE => vec![up, right],
            Tile::NW => vec![up, left],
            Tile::SW => vec![down, left],
            Tile::SE => vec![down, right],
            Tile::Ground => vec![],
            Tile::Start => vec![up, down, left, right],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Finder {
    current: (usize, usize),
    previous: (usize, usize),
}
impl Finder {
    fn advance(&mut self, grid: &HashMap<(usize, usize), Tile>) {
        let current_tile = grid.get(&self.current).unwrap();
        let options: Vec<_> = current_tile
            .connections(self.current)
            .into_iter()
            .filter(|con| con != &self.previous)
            .collect();

        assert_eq!(options.len(), 1);
        self.previous = self.current;
        self.current = dbg!(options[0]);
    }
}

fn compute(input: String) -> String {
    let grid: HashMap<(usize, usize), Tile> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Tile::from_str(&c.to_string()).unwrap())
                .enumerate()
                .collect::<Vec<_>>()
        })
        .enumerate()
        .fold(HashMap::new(), |mut acc, (row, tiles)| {
            for (col, tile) in tiles.into_iter() {
                acc.insert((row, col), tile);
            }
            acc
        });

    let start = grid
        .iter()
        .find(|(_, v)| v == &&Tile::Start)
        .unwrap()
        .0
        .to_owned();

    let finders: Vec<Finder> = Tile::Start
        .connections(start)
        .into_iter()
        .filter_map(|coord| {
            grid.get(&coord).and_then(|tile| {
                if tile != &Tile::Start {
                    // Happens if start is at the edge
                    Some((coord, tile))
                } else {
                    None
                }
            })
        })
        .filter_map(|(coord, tile)| {
            if tile.connections(coord).contains(&start) {
                Some(Finder {
                    previous: start,
                    current: coord,
                })
            } else {
                None
            }
        })
        .collect();

    assert_eq!(finders.len(), 2);
    let (mut a, mut b) = (finders[0], finders[1]);

    let mut steps = 1;

    loop {
        a.advance(&grid);
        b.advance(&grid);
        steps += 1;

        // If there is a case where they jump over each other, this fails
        // Not sure if that's possible
        if a.current == b.current {
            break steps;
        }
    }
    .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = fs::read_to_string("inputs/example1-1.txt").unwrap();
        assert_eq!("4", compute(input));
    }

    #[test]
    fn test_example2() {
        let input = fs::read_to_string("inputs/example1-2.txt").unwrap();
        assert_eq!("8", compute(input));
    }
}
