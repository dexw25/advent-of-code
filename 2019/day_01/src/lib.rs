#[cfg(test)]
mod tests {
    #[test]
    fn part_only() {
        assert_eq!(crate::part_one(&vec![100756]), 33583);
    }
    #[test]
    fn fuel_included() {
        assert_eq!(crate::part_two(&vec![100756]), 50346);
    }
}

// Implement the following algorithim:
//  "to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2"
// Use map() to efficiently collapse input to output
// Calculate fuel for parts not counting fuel mass
pub fn part_one(parts: &Vec<u32>) -> u32 {
    parts.iter().map(|&i| {
        if i > 6 {i / 3 - 2}
        else {0}
    }).sum()
}

// Include fuel mass as part of requirement
pub fn part_two(parts: &Vec<u32>) -> u32 {
    parts.iter().map(|&i| {
        if i < 9 {
            0
        } else {
            let fuel =  i / 3 - 2;
            fuel + fuel_for_mass(fuel)
        }
    }).sum()
}

fn fuel_for_mass(mass: u32) -> u32 {
    if mass < 9 {
        0
    } else {
        let fuel =  mass / 3 - 2;
        fuel + fuel_for_mass(fuel)
    }
}