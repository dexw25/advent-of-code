use core::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
pub struct Game {
    pub id: u32,
    pub trials: Vec<Trial>,
}

impl Game {
    #[must_use]
    pub fn valid(&self, constraints: &[Sample]) -> bool {
        self.trials.iter().all(|t| t.validate(constraints))
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.strip_prefix("Game ").ok_or("Missing prefix")?.split(':');

        let game_id = s
            .next()
            .ok_or("exhausted before game ID")?
            .parse::<u32>()
            .map_err(|_| "Failed to parse int")?;

        let trials: Result<Vec<Trial>, _> = s
            .next()
            .ok_or("Exhausted before trials")?
            .split(';')
            .filter(|t| !t.is_empty())
            .map(str::parse)
            .collect();
        let trials = trials?;

        Ok(Self {
            id: game_id,
            trials,
        })
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Trial {
    pub samples: Vec<Sample>,
}

impl Trial {
    fn validate(&self, constraints: &[Sample]) -> bool {
        self.samples.iter().all(|s| {
            constraints
                .iter()
                .any(|c| ((s.color == c.color) && (s.count <= c.count)))
        })
    }
}

impl FromStr for Trial {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trials = s
            .split(',')
            .filter(|t| !t.is_empty())
            .filter_map(|sub| sub.parse().ok())
            .collect();

        Ok(Self { samples: trials })
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Sample {
    pub color: Color,
    pub count: u32,
}
#[derive(PartialEq, Eq, Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Sample {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let n = tokens
            .next()
            .ok_or(format!("no number in {s}"))?
            .parse::<u32>()
            .map_err(|_| format!("Could not parse int from {s}"))?;
        let color = tokens.next().ok_or(format!("no color in {s}"))?;

        match color {
            "red" => Ok(Self {
                color: Color::Red,
                count: n,
            }),
            "green" => Ok(Self {
                color: Color::Green,
                count: n,
            }),
            "blue" => Ok(Self {
                color: Color::Blue,
                count: n,
            }),
            s => Err(format!("color {s} not recognized")),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::{Color, Game, Sample, Trial};

    #[test]
    fn validate() {
        assert!(!"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red;"
            .parse::<Game>()
            .unwrap()
            .valid(&[Sample {
                color: Color::Blue,
                count: 5
            }]));
        assert!("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red;"
            .parse::<Game>()
            .unwrap()
            .valid(&[
                Sample {
                    color: Color::Blue,
                    count: 5
                },
                Sample {
                    color: Color::Green,
                    count: 10,
                },
                Sample {
                    color: Color::Red,
                    count: 10
                }
            ]));
    }

    #[test]
    fn parse_game() {
        assert_eq!(
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red;"
                .parse::<Game>()
                .unwrap(),
            Game {
                id: 2,
                trials: vec![
                    Trial {
                        samples: vec![
                            Sample {
                                color: Color::Blue,
                                count: 1
                            },
                            Sample {
                                color: Color::Green,
                                count: 2
                            }
                        ]
                    },
                    Trial {
                        samples: vec![
                            Sample {
                                color: Color::Green,
                                count: 3
                            },
                            Sample {
                                color: Color::Blue,
                                count: 4
                            },
                            Sample {
                                color: Color::Red,
                                count: 1
                            }
                        ]
                    }
                ]
            }
        );
    }
}
