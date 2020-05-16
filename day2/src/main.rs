use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("day2/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let original_inputs:Vec<i32> = contents
        .split(",")
        .map(|text| text.trim())
        .map(|text| text.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut inputs = original_inputs.clone();

    inputs[1] = 12;
    inputs[2] = 2;

    interpret_intcode(&mut inputs);

    println!("Answer for day 2 - part 1 : {}", inputs[0]);

    for noun in 0..99  {
        for verb in 0..99 {
            let mut inputs = original_inputs.clone();
            inputs[1] = noun;
            inputs[2] = verb;


            interpret_intcode(&mut inputs);

            if inputs[0] == 19690720 {
                println!("Answer for day 2 - part 2 : {}", 100 * noun + verb);
            }
        }
    }

}

fn interpret_intcode(input: &mut Vec<i32>) {
    for i in (0..input.len()).step_by(4) {
        match input[i] {
            1 => {
                let input1 = input[input[(i + 1)] as usize];
                let input2 = input[input[(i + 2)] as usize];
                let dest = input[(i + 3) as usize] as usize;
                input[dest] = input1 + input2            }
            2 => {
                let input1 = input[input[(i + 1)] as usize];
                let input2 = input[input[(i + 2)] as usize];
                let dest = input[(i + 3) as usize] as usize;
                input[dest] = input1 * input2
            }
            99 => return,
            _ => panic!("Shouldn't happen"),
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_interpret_intcode() {
        let mut test1 = vec![1, 0, 0, 0, 99];
        interpret_intcode(&mut test1);

        let mut test2 = vec![2, 3, 0, 3, 99];
        interpret_intcode(&mut test2);

        let mut test3 = vec![2, 4, 4, 5, 99, 0];
        interpret_intcode(&mut test3);

        let mut test4 = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        interpret_intcode(&mut test4);

        assert_eq!(test1, vec![2, 0, 0, 0, 99]);
        assert_eq!(test2, vec![2, 3, 0, 6, 99]);
        assert_eq!(test3, vec![2, 4, 4, 5, 99, 9801]);
        assert_eq!(test4, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
