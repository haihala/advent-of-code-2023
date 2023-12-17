use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", compute(input));
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
    #[default]
    None,
}
impl Direction {
    fn is_opposite_to(&self, other: Direction) -> bool {
        match (self, other) {
            (Direction::North, Direction::South)
            | (Direction::South, Direction::North)
            | (Direction::West, Direction::East)
            | (Direction::East, Direction::West) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Edge {
    heading: Direction,
    tiles_since_last_turn: usize,
    pos: Position,
}

impl Default for Edge {
    fn default() -> Self {
        Self {
            heading: Direction::None,
            pos: Position::default(),
            tiles_since_last_turn: 0,
        }
    }
}

impl Edge {
    fn continue_to(&self, target: Position) -> Option<Edge> {
        let heading = self.pos.heading_to(target);
        if heading.is_opposite_to(self.heading) {
            return None;
        }

        let tiles_since_last_turn = if heading == self.heading {
            let combined_distance = self.tiles_since_last_turn + 1;
            if combined_distance > 3 {
                return None;
            }
            combined_distance
        } else {
            1
        };

        Some(Edge {
            heading,
            tiles_since_last_turn,
            pos: target,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Position {
    row: isize,
    col: isize,
}
impl Position {
    fn heading_to(&self, other: Position) -> Direction {
        match (self.row.cmp(&other.row), self.col.cmp(&other.col)) {
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => Direction::East,
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => Direction::West,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => Direction::South,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => Direction::North,

            (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => panic!("Same space"),
            _ => panic!("Not on axis"),
        }
    }

    fn neighbors(&self) -> Vec<Position> {
        // It's not a problem to yield invalid coordinates

        vec![
            Position {
                row: self.row - 1,
                ..self.to_owned()
            },
            Position {
                row: self.row + 1,
                ..self.to_owned()
            },
            Position {
                col: self.col - 1,
                ..self.to_owned()
            },
            Position {
                col: self.col + 1,
                ..self.to_owned()
            },
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Grid {
    nodes: HashMap<Position, usize>,
    bounds: Position,
}
impl Grid {
    fn from_costs(nodes: HashMap<Position, usize>) -> Self {
        Self {
            bounds: nodes
                .iter()
                .max_by_key(|(pos, _)| pos.row * pos.col)
                .unwrap()
                .0
                .to_owned(),
            nodes,
        }
    }
}

fn compute(input: String) -> String {
    let costs = parse(&input);
    let grid = Grid::from_costs(costs);
    let destination = grid.bounds;

    let mut explorers = vec![Edge::default()];
    let mut fastest_route: Option<usize> = None;
    let mut cache = explorers
        .clone()
        .into_iter()
        .map(|edge| (edge, 0))
        .collect::<HashMap<Edge, usize>>();

    while let Some(edge) = explorers.pop() {
        let new_cost = *cache.get(&edge).unwrap();

        if let Some(cost) = &fastest_route {
            if new_cost > *cost {
                // We've reached the goal faster than this, this can't be it
                continue;
            } else if edge.pos == destination {
                // Faster route
                fastest_route = Some(new_cost);
                continue;
            }
        } else if edge.pos == destination {
            // First one to reach the end
            fastest_route = Some(new_cost);
            continue;
        }

        let new_edges = edge
            .pos
            .neighbors()
            .into_iter()
            .filter_map(|pos| {
                grid.nodes
                    .get(&pos)
                    .and_then(|cost| Some((pos, new_cost + *cost)))
            })
            .filter_map(|(pos, cost)| {
                edge.continue_to(pos)
                    .and_then(|new_edge| Some((new_edge, cost)))
            })
            .filter(|(new_edge, cost)| {
                if let Some(val) = cache.get(&new_edge) {
                    if cost < val {
                        // Faster route
                        true
                    } else {
                        false
                    }
                } else {
                    // New route
                    true
                }
            })
            .collect::<Vec<_>>();

        for (new_edge, cost) in new_edges {
            cache.insert(new_edge.clone(), cost);
            explorers.push(new_edge);
        }
    }

    fastest_route.unwrap().to_string()
}

fn parse(input: &str) -> HashMap<Position, usize> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(col, char)| {
                (
                    Position {
                        row: row as isize,
                        col: col as isize,
                    },
                    char.to_digit(10).unwrap() as usize,
                )
            })
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("inputs/example.txt").unwrap();
        assert_eq!("102", compute(input));
    }
}
