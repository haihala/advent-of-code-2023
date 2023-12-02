use std::str::FromStr;

#[derive(Debug)]
pub struct Game {
    pub id: usize,
    pub rounds: Vec<Cubes>,
}
impl FromStr for Game {
    type Err = GameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rest = s.strip_prefix("Game ").ok_or(GameParseError)?;
        let id = rest
            .chars()
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let prefix = format!("{}: ", id);
        let rounds = rest
            .strip_prefix(prefix.as_str())
            .ok_or(GameParseError)?
            .split("; ")
            .map(|round| {
                let draws = round.split(", ").map(|draw| draw.split_once(" ").unwrap());

                let get_draw = |color: &str| {
                    draws
                        .clone()
                        .find_map(|draw| if draw.1 == color { Some(draw.0) } else { None })
                        .and_then(|val| val.parse::<usize>().ok())
                        .unwrap_or(0)
                };

                Cubes {
                    red: get_draw("red"),
                    blue: get_draw("blue"),
                    green: get_draw("green"),
                }
            })
            .collect();

        Ok(Game { id, rounds })
    }
}

#[derive(Debug)]
pub struct GameParseError;

#[derive(Debug, Default)]
pub struct Cubes {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}
impl Cubes {
    pub fn is_subset_of(&self, other: &Cubes) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
    pub fn max(&self, other: &Cubes) -> Self {
        Cubes {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
    pub fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}
