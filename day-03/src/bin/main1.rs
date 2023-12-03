use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", compute(input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Symbol {
    Value(usize),
    Anchor,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    row: i32,
    column: i32,
}
impl Coord {
    fn new(row: usize, column: usize) -> Coord {
        Coord {
            row: row as i32,
            column: column as i32,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Token {
    sym: Symbol,
    pos: Coord,
}

#[derive(Debug, Clone, Copy)]
struct BoundingBox {
    top_left: Coord,
    bottom_right: Coord,
}
impl BoundingBox {
    fn new(offset: Coord, value: usize) -> BoundingBox {
        let length = (value as f32).log10().floor() as i32 + 1;

        Self {
            top_left: Coord {
                row: offset.row - 1,
                column: offset.column - 1,
            },

            bottom_right: Coord {
                row: offset.row + 1,
                column: offset.column + length,
            },
        }
    }

    fn contains(&self, point: &Coord) -> bool {
        point.column >= self.top_left.column
            && point.column <= self.bottom_right.column
            && point.row >= self.top_left.row
            && point.row <= self.bottom_right.row
    }
}

fn compute(input: String) -> String {
    let tokens = input
        .lines()
        .enumerate()
        .flat_map(|(line_number, line)| parse_line(line_number, line))
        .collect::<Vec<_>>();

    let anchors: Vec<_> = tokens
        .iter()
        .filter_map(|element| match element.sym {
            Symbol::Anchor => Some(element.pos.clone()),
            _ => None,
        })
        .collect();

    tokens
        .iter()
        .filter_map(|element| match element.sym {
            Symbol::Value(value) => {
                let bb = BoundingBox::new(element.pos, value);

                if anchors.iter().any(|anchor| bb.contains(anchor)) {
                    Some(value)
                } else {
                    None
                }
            }
            _ => None,
        })
        .sum::<usize>()
        .to_string()
}

fn parse_line(line_number: usize, line: &str) -> Vec<Token> {
    let mut collector = vec![];

    let mut digits = vec![];
    for (char_number, next) in line.chars().enumerate() {
        if next.is_digit(10) {
            digits.push(next);
            continue;
        }

        if !digits.is_empty() {
            let offset = digits.len();
            let value = digits.drain(..).collect::<String>().parse().unwrap();

            collector.push(Token {
                sym: Symbol::Value(value),
                pos: Coord::new(line_number, char_number - offset),
            });
        }

        if next != '.' {
            collector.push(Token {
                sym: Symbol::Anchor,
                pos: Coord::new(line_number, char_number),
            });
        }
    }

    if !digits.is_empty() {
        let offset = digits.len();
        let value = digits.drain(..).collect::<String>().parse().unwrap();

        collector.push(Token {
            sym: Symbol::Value(value),
            pos: Coord::new(line_number, line.len() - offset),
        });
    }

    collector
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test_bounding_box_sizes() {
        for (value, size) in [
            (1, 2),
            (9, 2),
            (10, 3),
            (11, 3),
            (99, 3),
            (100, 4),
            (101, 4),
            (9999, 5),
            (10000, 6),
        ] {
            let bb = BoundingBox::new(Coord::new(4, 4), value);
            assert_eq!(bb.bottom_right.column - bb.top_left.column, size);
        }
    }

    #[test]
    fn test_bounding_box_contains_1x1() {
        // Ought to create a box so that top left is 3,3 and bottom right is 5, 5
        let bb = BoundingBox::new(Coord::new(4, 4), 1);

        // Center
        assert!(bb.contains(&Coord::new(4, 4)));

        // Corners
        assert!(bb.contains(&Coord::new(3, 3)));
        assert!(bb.contains(&Coord::new(5, 3)));
        assert!(bb.contains(&Coord::new(5, 5)));
        assert!(bb.contains(&Coord::new(3, 5)));

        // Above
        assert_eq!(false, bb.contains(&Coord::new(2, 4)));
        // Below
        assert_eq!(false, bb.contains(&Coord::new(6, 4)));
        // Left
        assert_eq!(false, bb.contains(&Coord::new(4, 2)));
        // Right
        assert_eq!(false, bb.contains(&Coord::new(4, 6)));

        // Top left
        assert_eq!(false, bb.contains(&Coord::new(2, 2)));
        // Bottom left
        assert_eq!(false, bb.contains(&Coord::new(6, 2)));
        // Top right
        assert_eq!(false, bb.contains(&Coord::new(2, 6)));
        // Bottom right
        assert_eq!(false, bb.contains(&Coord::new(6, 6)));
    }

    #[test]
    fn test_bounding_box_contains_2x1() {
        // Ought to create a box so that top left is 3,3 and bottom right is 5, 6
        let bb = BoundingBox::new(Coord::new(4, 4), 10);

        // Center
        assert!(bb.contains(&Coord::new(4, 4)));
        assert!(bb.contains(&Coord::new(4, 5)));

        // Corners
        assert!(bb.contains(&Coord::new(3, 3)));
        assert!(bb.contains(&Coord::new(5, 3)));
        assert!(bb.contains(&Coord::new(5, 6)));
        assert!(bb.contains(&Coord::new(3, 6)));

        // Above
        assert_eq!(false, bb.contains(&Coord::new(2, 4)));
        // Below
        assert_eq!(false, bb.contains(&Coord::new(6, 4)));
        // Left
        assert_eq!(false, bb.contains(&Coord::new(4, 2)));
        // Right
        assert_eq!(false, bb.contains(&Coord::new(4, 7)));

        // Top left
        assert_eq!(false, bb.contains(&Coord::new(2, 2)));
        // Bottom left
        assert_eq!(false, bb.contains(&Coord::new(6, 2)));
        // Top right
        assert_eq!(false, bb.contains(&Coord::new(2, 7)));
        // Bottom right
        assert_eq!(false, bb.contains(&Coord::new(6, 7)));
    }

    #[test]
    fn test_parse_line_basics() {
        assert_eq!(parse_line(5, ""), vec![]);
        assert_eq!(
            parse_line(5, "4"),
            vec![Token {
                sym: Symbol::Value(4),
                pos: Coord::new(5, 0)
            }]
        );
        assert_eq!(
            parse_line(5, "*"),
            vec![Token {
                sym: Symbol::Anchor,
                pos: Coord::new(5, 0)
            }]
        );
    }

    #[test]
    fn test_parse_line_pseudoline1() {
        assert_eq!(
            parse_line(5, "*.42"),
            vec![
                Token {
                    sym: Symbol::Anchor,
                    pos: Coord::new(5, 0)
                },
                Token {
                    sym: Symbol::Value(42),
                    pos: Coord::new(5, 2)
                }
            ]
        );
    }

    #[test]
    fn test_parse_line_pseudoline2() {
        assert_eq!(
            parse_line(5, "*.42..69"),
            vec![
                Token {
                    sym: Symbol::Anchor,
                    pos: Coord::new(5, 0)
                },
                Token {
                    sym: Symbol::Value(42),
                    pos: Coord::new(5, 2)
                },
                Token {
                    sym: Symbol::Value(69),
                    pos: Coord::new(5, 6)
                }
            ]
        );
    }

    #[test]
    fn test_parse_line_offsets() {
        assert_eq!(
            parse_line(5, ".*"),
            vec![Token {
                sym: Symbol::Anchor,
                pos: Coord::new(5, 1)
            }]
        );
        assert_eq!(parse_line(5, ".*."), parse_line(5, ".*"),);
    }

    #[test]
    fn test_example() {
        let example_input = fs::read_to_string("inputs/example1.txt").unwrap();
        assert_eq!(compute(example_input), "4361");
    }
}
