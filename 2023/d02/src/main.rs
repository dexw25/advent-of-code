use d02::{Color, Game, Sample};

#[allow(clippy::unwrap_used)]
fn main() {
    let input = include_str!("input.txt");

    let constraint = [
        Sample {
            color: d02::Color::Red,
            count: 12,
        },
        Sample {
            color: d02::Color::Green,
            count: 13,
        },
        Sample {
            color: d02::Color::Blue,
            count: 14,
        },
    ];

    dbg!(input
        .lines()
        .map(|l| l.parse::<Game>().unwrap())
        .filter_map(|g| {
            if g.valid(&constraint) {
                Some(g.id)
            } else {
                None
            }
        })
        .sum::<u32>());

    let p2 = input
        .lines()
        .map(|l| l.parse::<Game>().unwrap())
        .map(|g| {
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;
            g.trials
                .iter()
                .map(|t| {
                    t.samples
                        .iter()
                        .map(|s| match s.color {
                            Color::Blue => {
                                if min_blue < s.count {
                                    min_blue = s.count;
                                }
                            }
                            Color::Green => {
                                if min_green < s.count {
                                    min_green = s.count;
                                }
                            }
                            Color::Red => {
                                if min_red < s.count {
                                    min_red = s.count;
                                }
                            }
                        })
                        .count();
                })
                .count();
            min_red * min_blue * min_green
        })
        .sum::<u32>();

    dbg!(p2);
}
