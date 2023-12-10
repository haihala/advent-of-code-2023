use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs,
    iter::once,
    str::FromStr,
};

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

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Tile::NS => "|",
            Tile::EW => "-",
            Tile::NE => "L",
            Tile::NW => "J",
            Tile::SW => "7",
            Tile::SE => "F",
            Tile::Ground => ".",
            Tile::Start => "S",
        })
    }
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

    fn neighbors(coord: (usize, usize)) -> Vec<(usize, usize)> {
        // logically, they should probably call each other the other way
        Tile::Start.connections(coord)
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
        self.current = options[0];
    }
}

fn compute(input: String) -> String {
    let (grid, start) = parse_grid(input);
    let pipe_loop = follow_pipes(&grid, start);

    // Scale the whole thing up
    // This way there is a clear path out for anything not included
    let (scaled_grid, scaled_loop) = scaled_grid(&grid, &pipe_loop);

    if cfg!(test) {
        draw_grid(&grid);
        draw_grid(&scaled_grid);
    }

    let orig_areas = regions(&grid, &pipe_loop);
    let scaled_areas = regions(&scaled_grid, &scaled_loop);
    let scaled_inside = inside(&scaled_areas, &scaled_grid);

    orig_areas
        .into_iter()
        .filter(|area| {
            let point = area[0];
            scaled_inside.contains(&(point.0 * 2, point.1 * 2))
        })
        .fold(0, |acc, area| acc + area.len())
        .to_string()
}

fn inside(
    areas: &Vec<Vec<(usize, usize)>>,
    grid: &HashMap<(usize, usize), Tile>,
) -> Vec<(usize, usize)> {
    // This will find one of the inside areas
    // For the scaled version, it should be the only one.

    // Bottom left corner
    let max_coord = grid.keys().max_by_key(|(row, col)| row * col).unwrap();
    for area in areas {
        if !area.iter().any(|coord| {
            coord.0 == 0 || coord.0 == max_coord.0 || coord.1 == 0 || coord.1 == max_coord.1
        }) {
            return area.to_owned();
        }
    }
    // Didn't find an area that doesn't touch a wall.
    vec![]
}

fn draw_grid(grid: &HashMap<(usize, usize), Tile>) {
    let max_coord = grid.keys().max_by_key(|(row, col)| row * col).unwrap();
    for row in 0..max_coord.0 {
        println!(
            "{}",
            (0..max_coord.1)
                .map(|col| grid[&(row, col)].to_string())
                .collect::<String>()
        );
    }
}

fn regions(
    grid: &HashMap<(usize, usize), Tile>,
    pipe_loop: &[(usize, usize)],
) -> Vec<Vec<(usize, usize)>> {
    let mut areas: Vec<Vec<(usize, usize)>> = vec![];

    while let Some(fill_start) = grid
        .keys()
        .find(|k| !pipe_loop.contains(k) && !areas.iter().any(|area| area.contains(k)))
    {
        let mut flooded: HashSet<(usize, usize)> = vec![*fill_start].into_iter().collect();
        let mut checked = flooded.clone();
        let mut to_check: HashSet<(usize, usize)> = Tile::neighbors(*fill_start)
            .into_iter()
            .filter(|k| grid.contains_key(k))
            .collect();

        while let Some(next) = {
            let mut v: Vec<(usize, usize)> = to_check.into_iter().collect();
            let val = v.pop();
            to_check = v.into_iter().collect();
            val
        } {
            checked.insert(next);

            if !pipe_loop.contains(&next) && !areas.iter().any(|area| area.contains(&next)) {
                flooded.insert(next);

                to_check.extend(
                    Tile::neighbors(next)
                        .into_iter()
                        // The neighbors are not bounded by the
                        // size, just enought that they don't
                        // crash because usize went negative
                        .filter(|n| grid.contains_key(n))
                        // This will cause some redundant runs. Too bad.
                        .filter(|new| !checked.contains(new)),
                );
            }
        }
        areas.push(flooded.into_iter().collect());
    }
    areas
}

fn parse_grid(input: String) -> (HashMap<(usize, usize), Tile>, (usize, usize)) {
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
    (grid, start)
}

fn follow_pipes(
    grid: &HashMap<(usize, usize), Tile>,
    start: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut finder = Tile::Start
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
        .find_map(|(coord, tile)| {
            if tile.connections(coord).contains(&start) {
                Some(Finder {
                    previous: start,
                    current: coord,
                })
            } else {
                None
            }
        })
        .unwrap();

    let mut pipe_loop = vec![start];
    while finder.current != start {
        pipe_loop.push(finder.current);
        finder.advance(grid);
    }

    assert!(pipe_loop.contains(&start));
    pipe_loop
}

fn scaled_grid(
    grid: &HashMap<(usize, usize), Tile>,
    pipe_loop: &[(usize, usize)],
) -> (HashMap<(usize, usize), Tile>, Vec<(usize, usize)>) {
    // All interpolated pipes are straight
    let mut interp_ns = vec![];
    let mut interp_ew = vec![];

    let starting_point = pipe_loop[0];

    let scaled_loop = pipe_loop
        .iter()
        .chain(once(&starting_point)) // Added to bridge from last to first
        .map(|coord| (coord.0 * 2, coord.1 * 2)) // Scale
        .fold(vec![], |mut acc, coord| {
            // Interpolate

            let Some(last) = acc.last() else {
                acc.push(coord);
                return acc;
            };

            let ln: HashSet<_> = Tile::neighbors(*last).into_iter().collect();
            let nn: HashSet<_> = Tile::neighbors(coord).into_iter().collect();
            let overlap: Vec<_> = ln.intersection(&nn).cloned().collect();
            assert_eq!(overlap.len(), 1);
            let new_coord = *overlap.last().unwrap();

            if last.0 == coord.0 {
                interp_ew.push(new_coord);
            } else {
                interp_ns.push(new_coord);
            }

            acc.push(new_coord);
            acc.push(coord);
            acc
        });

    assert_eq!(scaled_loop.first().unwrap(), scaled_loop.last().unwrap());

    let scaled_grid = grid
        .iter()
        .flat_map(|(coord, tile)| {
            let orig_coord = (coord.0 * 2, coord.1 * 2);
            let orig = (orig_coord, tile.to_owned());
            let ext = vec![
                (orig_coord.0 + 1, orig_coord.1),
                (orig_coord.0, orig_coord.1 + 1),
                (orig_coord.0 + 1, orig_coord.1 + 1),
            ];

            // Realized after making this that this isn't necessary
            // Keeping it because it may make debugging look prettier
            vec![orig].into_iter().chain(ext.into_iter().map(|c| {
                (
                    c,
                    if interp_ew.contains(&c) {
                        Tile::EW
                    } else if interp_ns.contains(&c) {
                        Tile::NS
                    } else {
                        Tile::Ground
                    },
                )
            }))
        })
        .collect();

    (scaled_grid, scaled_loop)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = fs::read_to_string("inputs/example2-1.txt").unwrap();
        assert_eq!("4", compute(input));
    }

    #[test]
    fn test_example2() {
        let input = fs::read_to_string("inputs/example2-2.txt").unwrap();
        assert_eq!("8", compute(input));
    }

    #[test]
    fn test_example3() {
        let input = fs::read_to_string("inputs/example2-3.txt").unwrap();
        assert_eq!("10", compute(input));
    }
}
