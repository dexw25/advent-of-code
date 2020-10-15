use std::collections::HashSet;
use std::fmt;
use std::fs;

#[derive(Copy, Clone, PartialEq, Eq)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl Coord {
    fn sum(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Moon {
    pos: Coord,
    vel: Coord,
}

impl Moon {
    pub fn new(pos: Coord) -> Self {
        Moon {
            pos,
            vel: Coord { x: 0, y: 0, z: 0 },
        }
    }
}

struct Sim {
    bodies: Vec<Moon>,
}

enum Axes {
    X,
    Y,
    Z,
}

impl Sim {
    // Pass initial state by value and move it into this struct
    fn new(initial_state: Vec<Moon>) -> Self {
        Sim {
            bodies: initial_state,
        }
    }

    // Update gravity
    fn apply_gravity(&mut self) {
        for i in 0..self.bodies.len() {
            // Copy out position coord for processing
            let pos = self.bodies[i].pos;

            // Still not great, but better than a sort. Search all moons and calculate modifier by comparison and addition
            let mut accel_x: i64 = 0;
            let mut accel_y: i64 = 0;
            let mut accel_z: i64 = 0;
            for j in self.bodies.iter() {
                // add one for every body above this one, sub for every body below, ignore bodies at equal position
                accel_x += match pos.x - j.pos.x {
                    1..=std::i64::MAX => -1,
                    std::i64::MIN..=-1 => 1,
                    0 => 0,
                };
                // add one for every body above this one, sub for every body below, ignore bodies at equal position
                accel_y += match pos.y - j.pos.y {
                    1..=std::i64::MAX => -1,
                    std::i64::MIN..=-1 => 1,
                    0 => 0,
                };
                // add one for every body above this one, sub for every body below, ignore bodies at equal position
                accel_z += match pos.z - j.pos.z {
                    1..=std::i64::MAX => -1,
                    std::i64::MIN..=-1 => 1,
                    0 => 0,
                };
            }

            self.bodies[i].vel.x += accel_x;
            self.bodies[i].vel.y += accel_y;
            self.bodies[i].vel.z += accel_z;
        }
    }

    fn apply_velocity(&mut self) {
        for m in self.bodies.as_mut_slice() {
            m.pos.x += m.vel.x;
            m.pos.y += m.vel.y;
            m.pos.z += m.vel.z;
        }
    }

    // Step steps times
    fn step(&mut self, steps: usize) {
        for _i in 0..steps {
            // Apply gravity, mutate velocity based on position
            self.apply_gravity();

            // Apply velocity, mutate position based on velocity
            self.apply_velocity();
        }
    }

    // Total energy of system
    fn total_energy(&self) -> i64 {
        // Total energy is the sum of the absolute values of all components in the data format
        let mut total: i64 = 0;
        self.bodies
            .iter()
            .for_each(|x| total += x.pos.sum() * x.vel.sum());
        total
    }

    // Find steps until the system reaches it's first state, taking some assumptions
    //  - Each axis is 100% independent of the other two
    //  - Any loop point (that we care about) has zero velocity
    //  -
    fn loop_len(&mut self) -> u64 {
        // Step each single axis to find the loop period of each and then do LCM search
        // In theory this closure might be parallelized but this runs fast enough it doesn't need to be
        let single_axis_search = |istate: &Vec<Moon>, axis| -> u64 {
            let mut state: Vec<(i64, i64)> = vec![];

            for m in istate {
                // Select X Y or Z coord based on argument, this is thie only part that cares about axis
                state.push(match axis {
                    Axes::X => (m.pos.x, 0),
                    Axes::Y => (m.pos.y, 0),
                    Axes::Z => (m.pos.z, 0),
                });
            }
            let initstate = state.clone();
            let mut steps = 0;

            loop {
                // Apply gravity
                let state_ro = state.clone(); // immutable/read only clone so that original may be modified

                for (i, outer_s) in state.iter_mut().enumerate() {
                    let mut accel = 0;
                    let pos = outer_s.0; // 0 is position, 1 is vel
                    for (j, &s) in state_ro.iter().enumerate() {
                        if j != i {
                            // add one for every body above this one, sub for every body below, ignore bodies at equal position
                            use std::cmp::Ordering;
                            accel += match pos.cmp(&s.0) {
                                Ordering::Greater => -1,
                                Ordering::Less => 1,
                                Ordering::Equal => 0,
                            };
                        }
                    }
                    outer_s.1 += accel; // Update V
                }

                // Apply velocity
                for i in state.iter_mut() {
                    i.0 += i.1;
                }

                steps += 1;

                // Check if cycle got found
                let mut cycle = true;
                for i in 0..state.len() {
                    if state[i].1 != 0 || state[i].0 != initstate[i].0 {
                        cycle = false;
                        break;
                    }
                }
                if cycle {
                    break;
                }
            }
            steps
        };

        // Call the above closure on each axis
        let per_x = single_axis_search(&self.bodies, Axes::X);
        let per_y = single_axis_search(&self.bodies, Axes::Y);
        let per_z = single_axis_search(&self.bodies, Axes::Z);

        // Periods of each axis discovered, since motion on any individual axis is entirely periodic, we can find the LCM of these numbers
        //  And this is the period of the combination of all 3 subsystems
        let max_per = per_x.max(per_y.max(per_z));

        let p = primes(max_per);

        // LCM algorithm, decompose the 3 inputs into prime factors
        let x_dec = decompose(per_x, &p);
        let y_dec = decompose(per_y, &p);
        let z_dec = decompose(per_z, &p);

        // LCM is simply the result of the max of each prime factor, recomposed
        let mut res: Vec<u32> = Vec::with_capacity(p.len());
        for _i in 0..p.len() {
            res.push(0);
        } // initialize
          // for each prime slot store max of all 3 inputs
        for i in 0..res.len() {
            res[i] = x_dec[i].max(y_dec[i].max(z_dec[i]));
        }

        // Recompose
        let mut ret = 1;
        for (i, v) in res.iter().enumerate() {
            ret *= p[i].pow(*v);
        }
        ret
    }
}

// init from initial state in this form: '<x=14, y=2, z=8>', wierd things will happen if any members are missing, whitespace is ignored
impl From<&str> for Moon {
    fn from(val: &str) -> Self {
        // Trim braces and split into variables and data
        let mut s = val
            .trim_matches(|c| c == '<' || c == '>')
            .split_terminator(|c| c == ',' || c == '=');
        let mut ret = Coord { x: 0, y: 0, z: 0 };
        // Consume all tokens from parsed string
        while let Some(axis) = s.next() {
            match axis.trim() {
                "x" => ret.x = s.next().unwrap().parse::<i64>().unwrap(),
                "y" => ret.y = s.next().unwrap().parse::<i64>().unwrap(),
                "z" => ret.z = s.next().unwrap().parse::<i64>().unwrap(),
                &_ => panic!("Unexpected character {} found in initializer string", axis),
            }
        }
        Moon::new(ret)
    }
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<P: ({}, {}, {}), V: ({}, {}, {})>",
            self.pos.x, self.pos.y, self.pos.z, self.vel.x, self.vel.y, self.vel.z
        )
    }
}

// Random math
fn primes(top: u64) -> Vec<u64> {
    let mut multiples: HashSet<u64> = HashSet::new();
    let mut ret: Vec<u64> = Vec::new();
    for i in 2..top / 2 {
        // Make sure i is not a multiple of a prime we've seen already
        if multiples.contains(&i) {
            continue;
        } else {
            // else i is a new prime
            ret.push(i);
            let mut acc = i;
            // Cache all multiples of i in hash set
            loop {
                if acc > top / 2 {
                    break;
                }
                acc += i;
                multiples.insert(acc);
            }
        }
    }
    ret
}

// Decompose n to prime factors
fn decompose(mut n: u64, primes: &[u64]) -> Vec<u32> {
    let mut ret: Vec<u32> = Vec::with_capacity(primes.len());
    for _i in 0..primes.len() {
        ret.push(0);
    } // init with zeros
    let mut i = primes.len() - 1;
    while n > 1 {
        // if primes[i] divides n, add one to count and reduce n
        if n % primes[i] == 0 {
            n /= primes[i];
            ret[i] += 1;
        } else {
            // Else if remainder, dec i
            i -= 1;
        }
    }

    ret
}

fn main() -> std::io::Result<()> {
    let buf = &fs::read("./initstate.txt")?;

    // Initialize sim with state from file: For each line create a new moon object and make a vector of them all
    let mut sim = Sim::new(
        std::str::from_utf8(buf)
            .unwrap()
            .trim()
            .lines()
            .map(Moon::from)
            .collect(),
    );

    sim.step(1000);

    // //Print total energy in system
    println!("Total energy after 1000 steps: {}", sim.total_energy());

    // Reset sim
    let mut sim = Sim::new(
        std::str::from_utf8(buf)
            .unwrap()
            .trim()
            .lines()
            .map(Moon::from)
            .collect(),
    );

    println!("{} steps until loop", sim.loop_len());

    Ok(())
}
