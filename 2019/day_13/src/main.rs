use intcode::prog_from_file;
use intcode::IntcodeComp;
use std::convert::From;
use std::convert::TryInto;
use std::fmt;
use std::io::Read;

#[derive(Hash, PartialEq, Eq, Debug)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(PartialEq, Clone)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl From<i64> for Tile {
    fn from(src: i64) -> Self {
        match src {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("{} not a valid tile", src),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Tile::*;
        let out = match self {
            Empty => " ",
            Wall => "|",
            Block => "#",
            Paddle => "-",
            Ball => "0",
        };
        write!(f, "{}", out)?;
        Ok(())
    }
}

struct Game {
    frame: Vec<Tile>,
    cpu: IntcodeComp,
    score: i64,
    height: usize,
    width: usize,
}

impl Game {
    pub fn new(width: usize, height: usize, prog: &Vec<i64>) -> Game {
        // frame buffer is just a list of pixels, modulo index for rows
        let frame: Vec<Tile> = vec![Tile::Empty; width * height];
        let cpu = IntcodeComp::new(prog);
        let score: i64 = 0;
        Game {
            frame,
            cpu,
            score,
            height,
            width,
        }
    }

    // There's a bug in the game program that clears the ball position immediately before taking input
    //  So there is special logic in here to defer clearing the ball until after input is taken
    pub fn game_loop(&mut self, _ai: bool) {
        let mut ball_pos = Coord { x: 0, y: 0 };
        let mut defer_clear = false;
        loop {
            match self.cpu.eval_async() {
                true => {
                    // Update framebuf
                    while self.cpu.output_available() > 3 {
                        let x = self.cpu.output().unwrap();
                        let y = self.cpu.output().unwrap();
                        if x < 0 || y == 0 {
                            // -1,0 is score update
                            self.score = self.cpu.output().unwrap();
                        } else {
                            // Else coordinates should fit in frame buffer
                            let id = Tile::from(self.cpu.output().unwrap());
                            assert!(x < self.width as i64);
                            assert!(y < self.height as i64);

                            // Track ball position and defer clearing so that ball is rendered
                            if self.frame[(x as usize) + (y as usize) * self.width] == Tile::Ball
                                && id == Tile::Empty
                            {
                                defer_clear = true;
                                ball_pos.x = x;
                                ball_pos.y = y;
                            } else {
                                self.frame[(x as usize) + (y as usize) * self.width] = id;
                            }
                        }
                    }

                    // Render framebuf then score
                    for y in 0..self.height {
                        for x in 0..self.width {
                            print!("{}", self.frame[x + y * self.width])
                        }
                        println!();
                    }
                    println!(
                        "Score: {}\r\nEnter Input, A|a for left, D|d for right, S|s for stay",
                        self.score
                    );
                    if defer_clear {
                        self.frame[(ball_pos.x as usize) + (ball_pos.y as usize) * self.width] =
                            Tile::Empty;
                        defer_clear = false;
                    }

                    // Loop until an input byte from stdin makes sense
                    let mut buf = [0; 1];
                    let key = loop {
                        match std::io::stdin().read(&mut buf) {
                            Ok(1) => match buf[0] as char {
                                'a' | 'A' => break -1,
                                's' | 'S' => break 0,
                                'd' | 'D' => break 1,
                                _c => continue,
                            },
                            _n => continue,
                        }
                    };

                    // Feed input to program
                    self.cpu.input(key);
                }
                // Program breaks without asking for input when game is over
                false => {
                    break;
                }
            }
        }
    }
}

fn main() {
    let mut prog = prog_from_file("game.txt");
    let mut comp = IntcodeComp::new(&prog);
    comp.run_all();

    // Output is in units of 3, X coord, Y coord, and id for what is at that tile
    // Count total squares for part 1 check
    let mut squares = 0;
    let mut height = 0;
    let mut width = 0;
    while comp.output_available() > 3 {
        let x = comp.output().unwrap();
        let y = comp.output().unwrap();
        let id = Tile::from(comp.output().unwrap());

        // Total squares
        if id == Tile::Block {
            squares += 1
        };

        // Also keep track of window size for part 2, rather than hardcode window size
        //  Add one here because these metrics need to be a count of lines, and need to count line 0
        if width < x + 1 {
            width = x + 1
        };
        if height < y + 1 {
            height = y + 1
        };
    }
    drop(comp);

    println!("Number of squares: {}", squares);

    // Set address 0 to 2 for free play, and reinitialize computer
    prog[0] = 2;
    let mut g = Game::new(
        (width as usize).try_into().unwrap(),
        (height as usize).try_into().unwrap(),
        &prog,
    );

    // Play the game
    g.game_loop(false);
}
