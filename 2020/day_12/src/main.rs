mod data;

struct Ferry {
    bearing: i32, // Bearing in degrees, 0 is east (because honestly it doesnt matter)
    pos: (i32, i32),
    way: (i32, i32), // RELATIVE TO SHIP POSITION
}

impl Ferry {
    fn new() -> Ferry{
        Ferry {
            bearing: 0, pos: (0, 0), way: (10, 1)
        }
    }

    fn manhattan_dist(&self) -> u32 {
         (self.pos.0.abs() + self.pos.1.abs()) as u32
    }

    // Move the ship cursor the specified number of units in the given angle
    fn sail(&mut self, mut bearing: i32, distance: i32) {
        if bearing < 0 {
            bearing += 360; // Only implement positive angles
        }
        match bearing {
            0 => {self.pos.0 += distance},
            90 => {self.pos.1 += distance},
            180 => {self.pos.0 -= distance},
            270 => {self.pos.1 -= distance}
            _ => {panic!("Bad heading {}, miss a mod?", bearing)}
        }
    }

    // Sail in the current direction
    fn sail_forward(&mut self, distance: i32) {
        self.sail(self.bearing, distance);
    }

    // Sail to waypoint, the given number of times
    fn sail_to_waypoint(&mut self, times: i32) {
        self.pos.0 += self.way.0 * times;
        self.pos.1 += self.way.1 * times;
    }

    // Execute all steps and return count of steps run (no waypoint)
    fn execute_steps(&mut self, input: &str) -> u32{
        let mut counter=0;

        for step in input.lines() {
            self.execute(step);
            counter += 1;
        }

        counter
    }

    // Execute a single step (no waypoint)
    fn execute(&mut self, input: &str) {
        let distance = input[1..].parse::<i32>().unwrap();
        match &input[0..1]{
            "N" => {self.sail(90, distance)}
            "S" => {self.sail(270, distance)},
            "E" => {self.sail(0, distance)},
            "W" => {self.sail(180, distance)},
            "F" => {self.sail_forward(distance)},
            "L" => {self.bearing = (self.bearing + distance) % 360},
            "R" => {self.bearing = (self.bearing - distance) % 360},
            _ => {panic!("Bad input")}
        };
    }

    // navigate with waypoints
    fn execute_steps_with_waypoint(&mut self, input: &str) -> u32 {
        let mut counter = 0;

        for step in input.lines() {
            self.execute_with_waypoint(step);
            counter += 1;
        }

        counter
    }

    // Transform a waypoint, rotate around ship
    //  The real mistake was doing this too late at night to understand vector transformations
    fn wp_transform(&mut self, mut dir: i32) {
        if dir < 0 {dir += 360;}

        self.way = match dir {
            0 => { self.way },
            90 => {(-self.way.1, self.way.0)},
            180 => {(-self.way.0, -self.way.1)},
            270 => {(self.way.1, -self.way.0)},
            _ => {unreachable!()} // Input is such that this can't really happen
        }
    }

    // Execute a single step with waypoints factored in
    fn execute_with_waypoint(&mut self, input: &str) {
        let distance = input[1..].parse::<i32>().unwrap();
        match &input[0..1]{
            "N" => {self.way.1 += distance}
            "S" => {self.way.1 -= distance},
            "E" => {self.way.0 += distance},
            "W" => {self.way.0 -= distance},
            "F" => {self.sail_to_waypoint(distance)},
            "L" => {self.wp_transform(distance)}, // transform waypoint, left turns are positive
            "R" => {self.wp_transform(-distance)},
            _ => {panic!("Bad input")}
        };
    }
}

fn day_12_1(input: &str) -> u32 {
    let mut f = Ferry::new();

    f.execute_steps(input);

    f.manhattan_dist()
}

fn day_12_2(input: &str) -> u32 {
    let mut f = Ferry::new();

    f.execute_steps_with_waypoint(input);

    f.manhattan_dist()
}

fn main() {
    println!("p1: {}", day_12_1(data::DATA));
    println!("p2: {}", day_12_2(data::DATA));
}

#[test]
fn test1() {
    let input: &str = "F10
N3
F7
R90
F11";

    assert_eq!(day_12_1(input), 25);
}
#[test]
fn test2() {
    let input: &str = "F10
N3
F7
R90
F11";

    assert_eq!(day_12_2(input), 286);
}

// make sure rotation is valid as expected for given angles
#[test]
fn test_rotate() {
    let mut f = Ferry{way: (5, 10), pos: (0, 0), bearing: 0};

    f.wp_transform(90);
    assert_eq!(f.way, (-10, 5));

    f.wp_transform(180);
    assert_eq!(f.way, (10, -5));

    f.wp_transform(270);
    assert_eq!(f.way, (-5, -10));
}

// Waypoint test
#[test]
fn test_wp() {
    let mut f = Ferry{way: (5, 10), pos: (0, 0), bearing: 0};

    f.sail_to_waypoint(10);
    assert_eq!(f.pos, (50, 100));
}