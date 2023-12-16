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

    fn advance(&self, bounds: Position) -> Option<Beam> {
        match self.dir {
            Direction::Up => self.up(),
            Direction::Down => self.down(bounds),
            Direction::Left => self.left(),
            Direction::Right => self.right(bounds),
        }
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

    let tiles = get_tiles(&grid);

    visualize(&tiles, grid.bounds);

    tiles
        .into_iter()
        .map(|beam| beam.pos)
        .collect::<HashSet<_>>()
        .len()
        .to_string()
}

fn get_tiles(grid: &Grid) -> Vec<Beam> {
    let starting_beam = Beam {
        pos: Position { row: 0, col: 0 },
        dir: Direction::Right,
    };

    let mut explored = vec![];
    let mut explorers = vec![starting_beam];
    while let Some(explorer) = explorers.pop() {
        explored.push(explorer);

        let mut next = match (grid.map.get(&explorer.pos), explorer.dir) {
            (None, _)
            | (Some(Tile::VerticalSplitter), Direction::Up | Direction::Down)
            | (Some(Tile::HorizontalSplitter), Direction::Right | Direction::Left) => {
                // Empty space or at least one behaving as such

                if let Some(direct_descendant) = explorer.advance(grid.bounds) {
                    vec![direct_descendant]
                } else {
                    // Not in bounds
                    vec![]
                }
            }
            (Some(tile), _) => tile.outputs(explorer, grid.bounds),
        }
        .into_iter()
        .filter(|beam| !explored.contains(beam)) // Not already explored
        .filter(|beam| !explorers.contains(beam)) // Not already going to get explored
        .collect();

        explorers.append(&mut next);
    }

    explored
}

fn visualize(beams: &[Beam], bounds: Position) {
    let tiles: Vec<Position> = beams.into_iter().map(|beam| beam.pos).collect();

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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("inputs/example.txt").unwrap();
        assert_eq!("46", compute(input));
    }
}
