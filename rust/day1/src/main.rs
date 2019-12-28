fn fuel_required_for_module(mass: i32) -> i32 {
    mass / 3 - 2
}

fn total_fuel_required_for_module(module: i32) -> i32 {
    let mut fuel = fuel_required_for_module(module);
    let mut total = fuel;
    loop {
        fuel = fuel_required_for_module(fuel);
        if fuel > 0 {
            total += fuel;
        } else {
            break;
        }
    }
    total
}


fn main() {

    let input: [i32; 100] = [
        129880,
            115705,
            118585,
            124631,
            81050,
            138183,
            61173,
            95354,
            130788,
            89082,
            75554,
            110104,
            140528,
            71783,
            125889,
            126602,
            73089,
            76822,
            51774,
            85940,
            81004,
            149584,
            145921,
            105570,
            142370,
            80823,
            147779,
            115651,
            70250,
            67763,
            128192,
            51298,
            134963,
            73510,
            90976,
            141216,
            65134,
            140468,
            143998,
            101711,
            88477,
            53335,
            138328,
            141186,
            149804,
            64950,
            53107,
            54648,
            97557,
            85927,
            125038,
            80514,
            64912,
            140591,
            114229,
            57089,
            123464,
            127572,
            137169,
            146550,
            51138,
            115504,
            128034,
            147244,
            108107,
            101205,
            51498,
            136829,
            140171,
            59441,
            144489,
            139384,
            145841,
            96771,
            116821,
            88599,
            126780,
            65012,
            67621,
            129699,
            149639,
            97590,
            147527,
            117462,
            146709,
            60527,
            107643,
            92956,
            72177,
            92285,
            62475,
            63099,
            66904,
            77268,
            62945,
            134364,
            106924,
            117842,
            130016,
            123712,
    ];


    let mut total_fuel = 0;
    for module in input.iter() {
        total_fuel += fuel_required_for_module(*module);
    }

    println!("Answer for day 1 part 1 {}", total_fuel);


    total_fuel = 0;
    for module in input.iter() {
        total_fuel += total_fuel_required_for_module(*module);
    }

    println!("Answer for day 1 part 2 {}", total_fuel);
}


#[cfg(test)]
mod tests {

    use super::*;

    fn test_fuel_required_for_module(mass: i32, expected: i32) {
        println!("for mass of {} expected {} got {}", mass, expected, fuel_required_for_module(mass));
        assert_eq!(expected, fuel_required_for_module(mass));
    }

    fn test_total_fuel_required_for_module(mass: i32, expected: i32) {
        println!("for mass of {} expected {} got {}", mass, expected, total_fuel_required_for_module(mass));
        assert_eq!(expected, total_fuel_required_for_module(mass));
    }

    #[test]
    fn test_fuel() {
        test_fuel_required_for_module(12, 2);
        test_fuel_required_for_module(14, 2);
        test_fuel_required_for_module(1969, 654);
        test_fuel_required_for_module(100756, 33583);
    }

    #[test]
    fn total_test_fuel() {
        test_total_fuel_required_for_module(14, 2);
        test_total_fuel_required_for_module(1969, 966);
        test_total_fuel_required_for_module(100756, 50346);
    }
}
