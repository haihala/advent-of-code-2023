use std::{
    collections::{HashMap, HashSet},
    fs,
    str::FromStr,
};

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", compute(input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    dir: Direction,
    pos: Position,
}
impl Beam {
    fn up(&self) -> Option<Beam> {
        if self.pos.row == 0 {
            None
        } else {
            Some(Beam {
                dir: self.dir,
                pos: Position {
                    row: self.pos.row - 1,
                    col: self.pos.col,
                },
            })
        }
    }
    fn down(&self, bounds: Position) -> Option<Beam> {
        if self.pos.row == bounds.row - 1 {
            None
        } else {
            Some(Beam {
                dir: self.dir,
                pos: Position {
                    row: self.pos.row + 1,
                    col: self.pos.col,
                },
            })
        }
    }
    fn left(&self) -> Option<Beam> {
        if self.pos.col == 0 {
            None
        } else {
            Some(Beam {
                dir: self.dir,
                pos: Position {
                    row: self.pos.row,
                    col: self.pos.col - 1,
                },
            })
        }
    }
    fn right(&self, bounds: Position) -> Option<Beam> {
        if self.pos.col == bounds.col - 1 {
            None
        } else {
            Some(Beam {
                dir: self.dir,
                pos: Position {
                    row: self.pos.row,
                    col: self.pos.col + 1,
                },
            })
        }
    }

    fn step(&self, bounds: Position, direction: Direction) -> Option<Beam> {
        match direction {
            Direction::Up => self.up(),
            Direction::Down => self.down(bounds),
            Direction::Left => self.left(),
            Direction::Right => self.right(bounds),
        }
    }

    fn advance(&self, bounds: Position) -> Option<Beam> {
        self.step(bounds, self.dir)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    HorizontalSplitter, // -
    VerticalSplitter,   // |

    ForwardsMirror, // Forwards slash /
    BackMirror,     // Back slash \
}
impl Tile {
    fn outputs(&self, beam: Beam, bounds: Position) -> Vec<Beam> {
        let left = beam.left().map(|b| Beam {
            dir: Direction::Left,
            ..b
        });
        let right = beam.right(bounds).map(|b| Beam {
            dir: Direction::Right,
            ..b
        });
        let up = beam.up().map(|b| Beam {
            dir: Direction::Up,
            ..b
        });
        let down = beam.down(bounds).map(|b| Beam {
            dir: Direction::Down,
            ..b
        });

        match (self, beam.dir) {
            (Tile::HorizontalSplitter, Direction::Down | Direction::Up) => {
                vec![left, right]
            }
            (Tile::VerticalSplitter, Direction::Left | Direction::Right) => {
                vec![up, down]
            }
            (Tile::ForwardsMirror, Direction::Right) | (Tile::BackMirror, Direction::Left) => {
                vec![up]
            }
            (Tile::ForwardsMirror, Direction::Left) | (Tile::BackMirror, Direction::Right) => {
                vec![down]
            }
            (Tile::ForwardsMirror, Direction::Up) | (Tile::BackMirror, Direction::Down) => {
                vec![right]
            }
            (Tile::ForwardsMirror, Direction::Down) | (Tile::BackMirror, Direction::Up) => {
                vec![left]
            }
            // This shouldn't happen, as it gets checked earlier
            (tile, dir) => panic!("Trying to pass through {:?} in direction {:?}", tile, dir),
        }
        .into_iter()
        .flatten()
        .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Grid {
    map: HashMap<Position, Tile>,
    bounds: Position,
}
impl FromStr for Grid {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Grid {
            bounds: Position {
                row: input.lines().count(),
                col: input.lines().next().unwrap().len(),
            },
            map: input
                .lines()
                .enumerate()
                .flat_map(|(row, line)| {
                    line.chars().enumerate().filter_map(move |(col, char)| {
                        if char == '.' {
                            return None;
                        }

                        Some((
                            Position { row, col },
                            match char {
                                '-' => Tile::HorizontalSplitter,
                                '|' => Tile::VerticalSplitter,
                                '/' => Tile::ForwardsMirror,
                                '\\' => Tile::BackMirror,
                                other => panic!("Char '{}' in input", other),
                            },
                        ))
                    })
                })
                .collect(),
        })
    }
}

fn compute(input: String) -> String {
    let grid = Grid::from_str(&input).unwrap();

    let start_beams = starting_beams(grid.bounds);
    let max = get_max_coverage(&grid, start_beams);

    max.len().to_string()
}

fn get_max_coverage(grid: &Grid, start_beams: Vec<Beam>) -> HashSet<Position> {
    let mut best = HashSet::new();
    let mut cache = HashMap::new(); // This is here because it must be global for all of the
                                    // starting points

    for start_point in start_beams {
        let mut explorers: Vec<Beam> = vec![start_point];
        let mut visited: Vec<Beam> = vec![start_point];

        while let Some(current) = explorers.pop() {
            visited.push(current);

            let mut next: Vec<_> = cache
                .get(&current)
                .cloned()
                .unwrap_or_else(|| {
                    // cache miss
                    let out = match (grid.map.get(&current.pos), current.dir) {
                        (None, _)
                        | (Some(Tile::VerticalSplitter), Direction::Up | Direction::Down)
                        | (Some(Tile::HorizontalSplitter), Direction::Right | Direction::Left) => {
                            // Empty space or at least one behaving as such

                            if let Some(direct_descendant) = current.advance(grid.bounds) {
                                vec![direct_descendant]
                            } else {
                                // Not in bounds
                                vec![]
                            }
                        }
                        (Some(tile), _) => tile.outputs(current, grid.bounds),
                    };
                    cache.insert(current, out.clone());
                    out
                })
                .into_iter()
                .filter(|beam| !visited.contains(beam))
                .filter(|beam| !explorers.contains(beam))
                .collect();

            explorers.append(&mut next);
        }
        // fully_explored
        let tiles = visited
            .into_iter()
            .map(|beam| beam.pos)
            .collect::<HashSet<_>>();

        if tiles.len() > best.len() {
            best = tiles;
        }
    }

    best
}

fn starting_beams(bounds: Position) -> Vec<Beam> {
    let horizontal = (0..bounds.col).flat_map(|col| {
        vec![
            // Bottom up
            Beam {
                dir: Direction::Up,
                pos: Position {
                    row: bounds.col - 1,
                    col,
                },
            },
            // Top down
            Beam {
                dir: Direction::Down,
                pos: Position { row: 0, col },
            },
        ]
    });
    let vertical = (0..bounds.row).flat_map(|row| {
        vec![
            // ->
            Beam {
                dir: Direction::Right,
                pos: Position { col: 0, row },
            },
            // <-
            Beam {
                dir: Direction::Left,
                pos: Position {
                    col: bounds.col - 1,
                    row,
                },
            },
        ]
    });

    horizontal.chain(vertical).collect()
}

fn visualize(tiles: &[Position], bounds: Position) {
    for row in 0..bounds.row {
        println!(
            "{}",
            (0..bounds.col)
                .map(|col| if tiles.contains(&Position { row, col }) {
                    '#'
                } else {
                    '.'
                })
                .collect::<String>()
        );
    }
    println!("");
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("inputs/example.txt").unwrap();
        let grid = Grid::from_str(&input).unwrap();

        let start_beams = vec![Beam {
            dir: Direction::Down,
            pos: Position { row: 0, col: 3 },
        }];

        let best = get_max_coverage(&grid, start_beams);
        visualize(&(best.clone().into_iter().collect::<Vec<_>>()), grid.bounds);

        let out = best.len().to_string();

        assert_eq!("51", out);
    }

    #[test]
    fn max_example() {
        let input = fs::read_to_string("inputs/example.txt").unwrap();
        assert_eq!("51", compute(input));
    }

    #[test]
    fn test_starters() {
        for bounds in vec![
            (Position { row: 1, col: 1 }),
            (Position { row: 2, col: 1 }),
            (Position { row: 2, col: 2 }),
            (Position { row: 99, col: 99 }),
        ] {
            let expected_len = (bounds.row + bounds.col) * 2;
            let beams = starting_beams(bounds);
            let points: Vec<Position> = beams.clone().into_iter().map(|beam| beam.pos).collect();
            visualize(&points, bounds);

            assert_eq!(beams.len(), expected_len);
        }
    }

    #[test]
    fn test_movements() {
        let start = Beam {
            dir: Direction::Down,
            pos: Position { row: 5, col: 10 },
        };
        let sufficient_bounds = Position { row: 100, col: 100 };

        assert_eq!(start.up().unwrap().pos, Position { row: 4, col: 10 });
        assert_eq!(
            start.down(sufficient_bounds).unwrap().pos,
            Position { row: 6, col: 10 }
        );
        assert_eq!(start.left().unwrap().pos, Position { row: 5, col: 9 });
        assert_eq!(
            start.right(sufficient_bounds).unwrap().pos,
            Position { row: 5, col: 11 }
        );
    }
}
