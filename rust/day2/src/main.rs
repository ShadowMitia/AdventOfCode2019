struct Intcode(i32);

fn intcode_interpreter(intcode_program : &mut [i32]) -> &[i32] {
    let mut program_counter = 0;

    loop {
        let opcode = Intcode(intcode_program[program_counter]);

        match opcode {
            Intcode(1) => {
                let first_operand = intcode_program[intcode_program[program_counter+1] as usize];
                let second_operand = intcode_program[intcode_program[program_counter+2] as usize];
                let destination = intcode_program[program_counter+3];

                intcode_program[destination as usize] = first_operand + second_operand;
            }
            Intcode(2) => {
                let first_operand = intcode_program[intcode_program[program_counter+1] as usize];
                let second_operand = intcode_program[intcode_program[program_counter+2] as usize];
                let destination = intcode_program[program_counter+3];

                intcode_program[destination as usize] = first_operand * second_operand;
            }
            Intcode(99) => {
                break;
            }
            _ => {
                panic!("Error")
            }
        }

        program_counter += 4;
    }
    intcode_program
}

fn generate_program_for_input(noun:i32, verb:i32) -> [i32; 125] {
    let mut input:[i32;125] = [
        1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,5,19,23,2,10,23,27,1,27,5,31,2,9,31,35,1,35,5,39,2,6,39,43,1,43,5,47,2,47,10,51,2,51,6,55,1,5,55,59,2,10,59,63,1,63,6,67,2,67,6,71,1,71,5,75,1,13,75,79,1,6,79,83,2,83,13,87,1,87,6,91,1,10,91,95,1,95,9,99,2,99,13,103,1,103,6,107,2,107,6,111,1,111,2,115,1,115,13,0,99,2,0,14,0
    ];
    input[1] = noun;
    input[2] = verb;

    input
}

fn main() {
    let mut input = generate_program_for_input(12, 2);

    intcode_interpreter(&mut input);
    println!("Answer for day 2 part 1 {}", input[0]);

    for x in 1..100 {
        for y in 1..100 {
            let mut input = generate_program_for_input(x, y);
            intcode_interpreter(&mut input);
            if input[0] == 19690720 {
                println!("Answer for day 2 part 2 {}", 100 * x +y);            
            }
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    fn test_program(program: &mut [i32], expected: &[i32]) {
        intcode_interpreter(program);
        assert_eq!(program, expected);
    }

    #[test]
    fn test_all_programs() {
        let mut program_one = [1,0,0,0,99];
        let program_one_expected = [2, 0, 0, 0, 99];

        let mut program_two = [2,3,0,3,99];
        let program_two_expected = [2,3,0,6,99];

        let mut program_three = [2,4,4,5,99,0];
        let program_three_expected = [2,4,4,5,99,9801];

        let mut program_four = [1,1,1,4,99,5,6,0,99];
        let program_four_expected = [30,1,1,4,2,5,6,0,99];

        test_program(&mut program_one, &program_one_expected);
        test_program(&mut program_two, &program_two_expected);
        test_program(&mut program_three, &program_three_expected);
        test_program(&mut program_four, &program_four_expected);
    }

}
