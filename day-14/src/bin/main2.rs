use std::{
    collections::{HashMap, HashSet},
    fs,
    iter::once,
};

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", compute(input));
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Rock {
    Cube,
    Round,
}

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash)]
struct Point {
    row: usize,
    col: usize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    fn get_key(&self, point: &Point) -> usize {
        match self {
            Direction::North | Direction::South => point.row,
            Direction::East | Direction::West => point.col,
        }
    }

    fn reverse(&self) -> bool {
        match self {
            Direction::North | Direction::West => false,
            Direction::South | Direction::East => true,
        }
    }

    fn change_axis(&self, point: Point, new_value: usize) -> Point {
        match self {
            Direction::North | Direction::South => Point {
                row: new_value,
                col: point.col,
            },
            Direction::East | Direction::West => Point {
                row: point.row,
                col: new_value,
            },
        }
    }

    fn change_off_axis(&self, point: Point, new_value: usize) -> Point {
        match self {
            Direction::North | Direction::South => Point {
                row: point.row,
                col: new_value,
            },
            Direction::East | Direction::West => Point {
                row: new_value,
                col: point.col,
            },
        }
    }

    fn get_off_key(&self, point: &Point) -> usize {
        match self {
            Direction::North | Direction::South => point.col,
            Direction::East | Direction::West => point.row,
        }
    }
}

fn tilt(strinput: String, direction: Direction, range: Point) -> String {
    let input = collect_rocks(&strinput);

    // I'm kinda banking on this ending up in a non-repeating cycle and that memoization got me
    let lines: HashSet<usize> = input
        .keys()
        .map(|point| direction.get_off_key(point))
        .collect();

    let tilted = lines
        .into_iter()
        .flat_map(|line| {
            let mut inhabitants: Vec<_> = input
                .clone()
                .into_iter()
                .filter(|(point, _)| direction.get_off_key(point) == line)
                .collect();

            // Inhabitants to order
            inhabitants.sort_by_key(|(point, _)| direction.get_key(point));

            // Sort roundies based on the gap in cubes they belong to
            let chunks = inhabitants
                .into_iter()
                .fold(vec![(None, vec![])], |mut acc, rock| {
                    if rock.1 == Rock::Cube {
                        acc.push((Some(direction.get_key(&rock.0)), vec![]))
                    } else {
                        let last = acc.last_mut().unwrap();
                        last.1.push(rock.0);
                    }

                    acc
                });

            // Re-assemble structure
            if direction.reverse() {
                // reverse iteration order, use key of previous
                chunks
                    .into_iter()
                    .chain(once((None, vec![])))
                    .collect::<Vec<_>>()
                    .windows(2)
                    .map(|win| {
                        let (mut chunk, next) = (win[0].to_owned(), win[1].to_owned());
                        // Move the headings
                        chunk.0 = next.0;
                        chunk
                    })
                    .flat_map(|(cube_index, rounds)| {
                        if let Some(cube) = cube_index {
                            rounds
                                .into_iter()
                                .enumerate()
                                .map(|(index, point)| {
                                    (direction.change_axis(point, cube - index - 1), Rock::Round)
                                })
                                .chain(once((
                                    direction.change_off_axis(
                                        direction.change_axis(Point::default(), cube),
                                        line,
                                    ),
                                    Rock::Cube,
                                )))
                                .collect()
                        } else {
                            // last, no actual cube
                            rounds
                                .into_iter()
                                .enumerate()
                                .map(|(index, point)| {
                                    (
                                        direction.change_axis(
                                            point,
                                            direction.get_key(&range) - index - 1,
                                        ),
                                        Rock::Round,
                                    )
                                })
                                .collect::<Vec<_>>()
                        }
                    })
                    .collect::<Vec<_>>()
            } else {
                chunks
                    .into_iter()
                    .flat_map(|(cube_index, rounds)| {
                        if let Some(cube) = cube_index {
                            once((
                                direction.change_off_axis(
                                    direction.change_axis(Point::default(), cube),
                                    line,
                                ),
                                Rock::Cube,
                            ))
                            .chain(rounds.into_iter().enumerate().map(|(index, point)| {
                                // Point with index derived primary axis,
                                (direction.change_axis(point, index + cube + 1), Rock::Round)
                            }))
                            .collect()
                        } else {
                            // First bit, no actual rock there
                            rounds
                                .into_iter()
                                .enumerate()
                                .map(|(index, point)| {
                                    // Point with index derived primary axis,
                                    (direction.change_axis(point, index), Rock::Round)
                                })
                                .collect::<Vec<_>>()
                        }
                    })
                    .collect()
            }
        })
        .collect();

    pack_up_rocks(&tilted, range)
}

fn spin(mut state: String, cycles: usize) -> HashMap<Point, Rock> {
    let range = Point {
        row: state.lines().count(),
        col: state.lines().next().unwrap().len(),
    };

    let mut history = vec![];

    for i in 0..cycles {
        history.push(state.clone());
        // One 'cycle' is tilt north, west, south, east in that order
        state = tilt(state, Direction::North, range);
        state = tilt(state, Direction::West, range);
        state = tilt(state, Direction::South, range);
        state = tilt(state, Direction::East, range);

        if let Some(cycle_start) = history.iter().position(|step| step == &state) {
            // Cycle detected
            let cycle_length = (i + 1) - cycle_start;
            let remaining = cycles - cycle_start;
            let remainder = remaining % cycle_length;

            return collect_rocks(&history[cycle_start + remainder]);
        }
    }
    collect_rocks(&state)
}

// Deserialize
fn collect_rocks(input: &str) -> HashMap<Point, Rock> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(move |(col, ch)| {
                let point = Point { row, col };
                match ch {
                    '.' => None,
                    '#' => Some((point, Rock::Cube)),
                    'O' => Some((point, Rock::Round)),
                    _ => panic!("Invalid char {:?}", ch),
                }
            })
        })
        .collect()
}

// Serialize
fn pack_up_rocks(input: &HashMap<Point, Rock>, range: Point) -> String {
    (0..range.row)
        .map(|row| {
            (0..range.col)
                .map(|col| {
                    if let Some(rock) = input.get(&Point { row, col }) {
                        match rock {
                            Rock::Cube => '#',
                            Rock::Round => 'O',
                        }
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn load(input: HashMap<Point, Rock>, max_line: usize) -> usize {
    input
        .into_iter()
        .filter(|(_, rock)| rock == &Rock::Round)
        .map(|(point, _)| (max_line - point.row))
        .sum()
}

fn compute(input: String) -> String {
    let lines = input.lines().count();
    load(spin(input, 1000000000), lines).to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::repeat;

    fn visualize(rocks: &HashMap<Point, Rock>, range: Point) {
        let cubes = rocks.values().filter(|rock| rock == &&Rock::Cube).count();
        let rounds = rocks.values().filter(|rock| rock == &&Rock::Round).count();

        println!("{} cubes, {} rounds", cubes, rounds);
        dbg!(pack_up_rocks(rocks, range).lines().collect::<Vec<_>>());
        println!("");
    }

    #[test]
    fn test_spin() {
        let pre = fs::read_to_string("inputs/example_pre_tilt.txt").unwrap();
        let post = collect_rocks(&fs::read_to_string("inputs/example_spin1.txt").unwrap());

        let range = Point {
            row: pre.lines().count(),
            col: pre.lines().next().unwrap().len(),
        };

        let spun = spin(pre, 1);

        dbg!("Expected");
        visualize(&post, range);
        println!("{}", repeat("-").take(range.col).collect::<String>());
        dbg!("Actual");
        visualize(&spun, range);

        assert_eq!(post, spun);
    }

    #[test]
    fn test_example() {
        let input = fs::read_to_string("inputs/example_pre_tilt.txt").unwrap();
        assert_eq!("64", compute(input));
    }
}
