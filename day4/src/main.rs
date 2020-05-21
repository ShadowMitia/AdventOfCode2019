use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("day4/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let splitted: Vec<(&str, &str)> = contents
        .lines()
        .map(|s| {
            let t: Vec<&str> = s.split("-").collect();

            (t[0], t[1])
        })
        .collect();
    println!("{:?}", splitted);
    let (begin_range, end_range): (u32, u32) = (
        splitted[0].0.parse().unwrap(),
        splitted[0].1.parse().unwrap(),
    );

    let res: Vec<u32> = (begin_range..end_range)
        .into_iter()
        .filter(|number| {
            let password = number.to_string();
            let digits: Vec<u32> = password.chars().map(|c| c.to_digit(10).unwrap()).collect();

            let mut has_double = false;

            for index in 0..digits.len() {
                for prev_index in 0..index {
                    if digits[prev_index] > digits[index] {
                        return false;
                    }

                    if digits[prev_index] == digits[index] {
                        has_double = true;
                    }
                }
            }

            has_double
        })
        .collect();

    println!("Answer for day4 - part1 {}", res.len());

    let splitted: Vec<(&str, &str)> = contents
        .lines()
        .map(|s| {
            let t: Vec<&str> = s.split("-").collect();

            (t[0], t[1])
        })
        .collect();
    println!("{:?}", splitted);
    let (begin_range, end_range): (u32, u32) = (
        splitted[0].0.parse().unwrap(),
        splitted[0].1.parse().unwrap(),
    );

    let res: Vec<u32> = (begin_range..end_range)
        .into_iter()
        .filter(|number| {
            let password = number.to_string();
            let digits: Vec<u32> = password.chars().map(|c| c.to_digit(10).unwrap()).collect();

            let mut has_double = false;
            for index in 0..digits.len() {
                for prev_index in 0..index {
                    if digits[prev_index] > digits[index] {
                        return false;
                    }

                    if digits[prev_index] == digits[index] {
                        if index < digits.len() - 1 && digits[index+1] != digits[index]{
                            has_double = true;
                        } 
                    }
                }
            }

            has_double
        })
        .collect();

    println!("Answer for day4 - part2 {}", res.len());
}
