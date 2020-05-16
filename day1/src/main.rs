use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("day1/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let inputs: Vec<i32> = contents.lines().map(|text| text.parse().unwrap()).collect();

    println!(
        "Answer for day1 - part 1 {}",
        inputs
            .iter()
            .map(|&mass| get_fuel_for_mass(mass))
            .sum::<i32>()
    );

    println!(
        "Answer for day1 - part 2 {}",
        inputs
            .iter()
            .map(|&mass| get_fuel_for_mass_with_fuel(mass))
            .sum::<i32>()
    );
}

fn get_fuel_for_mass(mass: i32) -> i32 {
    let res = (f32::floor(mass as f32 / 3.0) - 2.0) as i32;
    if res < 0 {
        0
    } else {
        res
    }
}

fn get_fuel_for_mass_with_fuel(mass: i32) -> i32 {
    let res = get_fuel_for_mass(mass);
    if res <= 0 {
        0
    } else {
        res + get_fuel_for_mass_with_fuel(res)
    }
}

mod test {
    use super::*;
    #[test]
    fn test_get_fuel_for_mass() {
        assert_eq!(get_fuel_for_mass(12), 2);
        assert_eq!(get_fuel_for_mass(14), 2);
        assert_eq!(get_fuel_for_mass(1969), 654);
        assert_eq!(get_fuel_for_mass(100756), 33583);
    }

    #[test]
    fn test_get_fuel_for_mass_with_fuel() {
        assert_eq!(get_fuel_for_mass(12), 2);
        assert_eq!(get_fuel_for_mass_with_fuel(14), 2);
        assert_eq!(get_fuel_for_mass_with_fuel(1969), 966);
        assert_eq!(get_fuel_for_mass_with_fuel(100756), 50346);
    }
}
