use crate::data::*;

mod data;
// For each rate value, find how long from init it will arrive
fn day_13_1(init: u32, rates: &str) -> u32 {
    let mut soonest = u32::MAX;
    let mut soonest_id = 0;
    for bus_id in rates
        .split(',')
        .map(|period| period.parse::<u32>().or::<u32>(Ok(init)).unwrap())
    {
        // Calculate time of next instance of the given bus
        let next_bus = (init / bus_id + 1) * (bus_id);
        if next_bus < soonest {
            soonest = next_bus;
            soonest_id = bus_id;
        }
    }

    soonest_id * (soonest - init)
}

// Search occurrences of possible bus times until a sequential arrangement is found
fn day_13_2(input: &str) -> u64 {
    let rates: Vec<u64> = input
        .split(',')
        .map(|rate| rate.parse::<u64>().or::<u64>(Ok(1)).unwrap()) // Default to 1 so that the sieve falls through properly
        .collect();

    let mut inc = rates[0];
    let mut t: u64 = 0;

    // This is a CRT sieve: https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Computation
    // Step across the following buses, and bump the candidate first timestamp until the modulo test passes
    for (i, bus) in rates[1..].iter().enumerate() {
        // Bump first bus until the next bus's departure is 1 beyond the current
        while (t + (i + 1) as u64) % bus != 0 {
            // need to inc i as indices here are off by one
            t += inc;
        }

        // new increment is product of all previous bus rates, no sense searching invalid departure times
        inc *= bus;
    }
    // T now points to the first bus[0] such that the last bus leaves 1 after the bus before it, and so on

    t
}

fn main() {
    println!("p1: {}", day_13_1(DATA_I, DATA_RATES));
    println!("p2: {}", day_13_2(DATA_RATES));
}

#[test]
fn test1() {
    let i = 939;
    let rates = "7,13,x,x,59,x,31,19";

    assert_eq!(day_13_1(i, rates), 295);
}

#[test]
fn test2() {
    let rates = "7,13,x,x,59,x,31,19";
    assert_eq!(day_13_2(rates), 1068781);
}
