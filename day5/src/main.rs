use std::fs::File;
use std::io::prelude::*;

use std::io;

fn main() {
    let mut file = File::open("day5/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let original_inputs: Vec<i32> = contents
        .split(",")
        .map(|text| text.trim())
        .map(|text| text.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut inputs = original_inputs.clone();

    let output = interpret_intcode(&mut inputs, 1);

    println!("Answer for day 5 - part 1 : {:?}", output);

    let mut inputs = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]; // original_inputs.clone();
    let output = interpret_intcode(&mut inputs, 5);

    println!("Answer for day 5 - part 2 : {:?}", output);
}

fn interpret_intcode(input: &mut Vec<i32>, id: i32) -> Vec<i32> {
    let mut output = Vec::new();
    let mut index = 0;
    loop {
        let opcode_string = input[index as usize].to_string();
        let opcode = input[index as usize];
        let from = match opcode_string.len() {
            1 => 0,
            2 => 0,
            _ => opcode_string.len() - 2,
        };
        let op: u32 = opcode_string
            .get(from..opcode_string.len())
            .unwrap()
            .parse()
            .unwrap();
        match op {
            1 => {
                let mode_input1 = opcode & 0x4;
                index += 1;
                let next = index;
                let input1 = if mode_input1 == 0 {
                    input[input[next as usize] as usize]
                } else {
                    input[next as usize]
                };
                index += 1;
                let next = index;
                let mode_input2 = opcode & 0x8;
                let input2 = if mode_input2 == 0 {
                    input[input[next as usize] as usize]
                } else {
                    input[next as usize]
                };

                index += 1;
                let next = index;
                let dest = input[next as usize] as usize;
                let dest_to_modify = &mut input[dest];
                *dest_to_modify = input1 + input2;

                index += 1;
            }
            2 => {
                let mode_input1 = opcode & 0x4;
                index += 1;
                let next = index;
                let input1 = if mode_input1 == 0 {
                    input[input[next as usize] as usize]
                } else {
                    input[next as usize]
                };
                index += 1;
                let next = index;
                let mode_input2 = opcode & 0x8;
                let input2 = if mode_input2 == 0 {
                    input[input[next as usize] as usize]
                } else {
                    input[next as usize]
                };

                index += 1;
                let next = index;
                let dest = input[next as usize] as usize;
                input[dest] = input1 * input2;

                index += 1;
            }
            3 => {
                index += 1;
                let next = index;
                let dest = input[next as usize] as usize;
                input[dest] = id;

                index += 1;
            }
            4 => {
                index += 1;
                let next = index;
                let input = input[input[next as usize] as usize];
                output.push(input);

                index += 1;
            }
            5 => {
                let mode_input1 = opcode & 0x4;
                index += 1;
                let next = index;
                let input1 = if mode_input1 == 0 {
                    input[input[next as usize] as usize]
                } else {
                    input[next as usize]
                };
                index += 1;
                let next = index;
                let mode_input2 = opcode & 0x8;
                let input2 = if mode_input2 == 0 {
                    input[next as usize]
                } else {
                    next
                };

                if input1 != 0 {
                    index = input2;
                } else {
                    index += 1;
                }
            }
            6 => {
                let mode_input1 = opcode & 0x4;
                index += 1;
                let next = index;
                let input1 = if mode_input1 == 0 {
                    input[input[next as usize] as usize]
                } else {
                    input[next as usize]
                };
                index += 1;
                let next = index;
                let mode_input2 = opcode & 0x8;
                let input2 = if mode_input2 == 0 {
                    input[next as usize]
                } else {
                    next
                };

                if input1 == 0 {
                    index = input2;
                } else {
                    index += 1;
                }
            }
            7 => {
                let mode_input1 = opcode & 0x4;
                index += 1;
                let next = index;
                let input1 = if mode_input1 == 0 {
                    input[input[next as usize] as usize]
                } else {
                    input[next as usize]
                };
                index += 1;
                let next = index;
                let mode_input2 = opcode & 0x8;
                let input2 = if mode_input2 == 0 {
                    input[input[next as usize] as usize]
                } else {
                    input[next as usize]
                };
                index += 1;
                let next = index;
                let mode_input3 = opcode & 0x10;
                let input3 = if mode_input3 == 0 {
                    input[next as usize]
                } else {
                    next
                };

                if input1 < input2 {
                    input[input3 as usize] = 1;
                } else {
                    input[input3 as usize] = 0;
                }

                index += 1;
            }
            8 => {
                let mode_input1 = opcode & 0x4;
                index += 1;
                let next = index;
                let input1 = if mode_input1 == 0 {
                    input[input[next as usize] as usize]
                } else {
                    input[next as usize]
                };
                index += 1;
                let next = index;
                let mode_input2 = opcode & 0x8;
                let input2 = if mode_input2 == 0 {
                    input[input[next as usize] as usize]
                } else {
                    input[next as usize]
                };
                index += 1;
                let next = index;
                let mode_input3 = opcode & 0x10;
                let input3 = if mode_input3 == 0 {
                    input[next as usize]
                } else {
                    next
                };

                println!("{} {} {}", input1, input2, input3);

                if input1 == input2 {
                    input[input3 as usize] = 1;
                } else {
                    input[input3 as usize] = 0;
                }

                index += 1;
            }
            99 => break,
            a @ _ => panic!("Wrong or unimplemented opcode : {}", a),
        }
    }
    output
}

mod tests {
    use super::*;

    #[test]
    fn test_interpret_intcode() {
        let mut test1 = vec![1, 0, 0, 0, 99];
        interpret_intcode(&mut test1, 1);

        let mut test2 = vec![2, 3, 0, 3, 99];
        interpret_intcode(&mut test2, 1);

        let mut test3 = vec![2, 4, 4, 5, 99, 0];
        interpret_intcode(&mut test3, 1);

        let mut test4 = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        interpret_intcode(&mut test4, 1);

        assert_eq!(test1, vec![2, 0, 0, 0, 99]);
        assert_eq!(test2, vec![2, 3, 0, 6, 99]);
        assert_eq!(test3, vec![2, 4, 4, 5, 99, 9801]);
        assert_eq!(test4, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_interpret_intcode_conditonals() {
        let mut test1 = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let output1 = interpret_intcode(&mut test1, 8);

        let mut test2 = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let output2 = interpret_intcode(&mut test2, 8);

        let mut test3 = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let output3 = interpret_intcode(&mut test3, 8);

        let mut test4 = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let output4 = interpret_intcode(&mut test4, 8);

        println!("{:?}", test3);

        assert_eq!(output1, vec![1]);
        assert_eq!(output2, vec![0]);
        assert_eq!(output3, vec![1]);
        assert_eq!(output4, vec![0]);
    }
}
