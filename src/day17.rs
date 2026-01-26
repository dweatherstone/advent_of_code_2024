use itertools::Itertools;

#[derive(Default, Clone)]
pub struct Program {
    pub register_a: i64,
    register_b: i64,
    register_c: i64,
    initial_a: i64,
    initial_b: i64,
    initial_c: i64,
    instructions: Vec<Operation>,
    operands: Vec<u8>,
    pointer: usize,
    pub program: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operation {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Operation {
    fn apply(&self, operand: i64, program: &mut Program) -> Option<i64> {
        match self {
            Operation::Adv => dv(operand, program, Register::A),
            Operation::Bdv => dv(operand, program, Register::B),
            Operation::Cdv => dv(operand, program, Register::C),
            Operation::Bxl => bxl(operand, program),
            Operation::Bst => bst(operand, program),
            Operation::Jnz => jnz(operand, program),
            Operation::Bxc => bxc(program),
            Operation::Out => out(operand, program),
        }
    }
}

fn get_combo(operand: i64, program: &Program) -> i64 {
    match operand {
        0..=3 => operand,
        4 => program.register_a,
        5 => program.register_b,
        6 => program.register_c,
        _ => panic!("combo operand unknown: {operand}"),
    }
}

fn dv(operand: i64, program: &mut Program, register: Register) -> Option<i64> {
    let numerator = program.register_a;
    // The problem states the denominator is 2 to the power of the combo operand.
    // In Rust, x / 2^n is equivalent to x >> n.
    let shift_amount = get_combo(operand, program) as u32;

    // Perform the division via right-shift
    let result = numerator >> shift_amount;

    match register {
        Register::A => program.register_a = result,
        Register::B => program.register_b = result,
        Register::C => program.register_c = result,
    }
    None
}

fn bxl(operand: i64, program: &mut Program) -> Option<i64> {
    program.register_b ^= operand;
    None
}

fn bst(operand: i64, program: &mut Program) -> Option<i64> {
    program.register_b = get_combo(operand, program) % 8;
    None
}

fn jnz(operand: i64, program: &mut Program) -> Option<i64> {
    if program.register_a == 0 {
        return None;
    }
    program.pointer = operand as usize / 2;
    Some(0)
}

fn bxc(program: &mut Program) -> Option<i64> {
    program.register_b ^= program.register_c;
    None
}

fn out(operand: i64, program: &mut Program) -> Option<i64> {
    Some(get_combo(operand, program) % 8)
}

impl TryFrom<u8> for Operation {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Operation::Adv),
            1 => Ok(Operation::Bxl),
            2 => Ok(Operation::Bst),
            3 => Ok(Operation::Jnz),
            4 => Ok(Operation::Bxc),
            5 => Ok(Operation::Out),
            6 => Ok(Operation::Bdv),
            7 => Ok(Operation::Cdv),
            _ => Err("unknown operation number"),
        }
    }
}

enum Register {
    A,
    B,
    C,
}

pub fn parse_day17(lines: &[String]) -> Program {
    let mut register_a = 0;
    let mut register_b = 0;
    let mut register_c = 0;
    let mut instructions: Vec<Operation> = Vec::new();
    let mut operands: Vec<u8> = Vec::new();
    let mut program: Vec<u8> = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let (key, value) = line.split_once(':').expect("Missing colon in line");
        let value = value.trim();
        match key.trim() {
            "Register A" => register_a = value.parse::<i64>().expect("should be an integer"),
            "Register B" => register_b = value.parse::<i64>().expect("should be an integer"),
            "Register C" => register_c = value.parse::<i64>().expect("should be an integer"),
            "Program" => {
                program = value
                    .split(",")
                    .map(|v| v.trim().parse::<u8>().expect("should be a small integer"))
                    .collect();
                for (instruction, operand) in value.split(',').tuples() {
                    instructions.push(
                        instruction
                            .trim()
                            .parse::<u8>()
                            .expect("should be a small integer")
                            .try_into()
                            .expect("unknown operation opcode"),
                    );
                    operands.push(
                        operand
                            .trim()
                            .parse::<u8>()
                            .expect("should be a small integer"),
                    );
                }
            }
            _ => panic!("Unknown line in input"),
        }
    }

    Program {
        register_a,
        register_b,
        register_c,
        initial_a: register_a,
        initial_b: register_b,
        initial_c: register_c,
        instructions,
        operands,
        pointer: 0,
        program,
    }
}

pub fn run_program_day17_stage1(program: &mut Program) -> String {
    let mut output: Vec<i64> = Vec::new();
    while program.pointer < program.instructions.len() {
        let operation = program.instructions[program.pointer];
        let operand = program.operands[program.pointer] as i64;

        let result = operation.apply(operand, program);

        if operation == Operation::Out
            && let Some(res) = result
        {
            output.push(res);
        }
        if operation == Operation::Jnz && result.is_some() {
            continue;
        }
        program.pointer += 1;
    }

    output.iter().map(|val| val.to_string()).join(",")
}

pub fn get_register_a_day17_stage2(program: &Program) -> i64 {
    // We start searching for the LAST element of the program first.
    // solve(current_a, index_of_target_digit)
    match find_a(0, program.program.len() - 1, program) {
        Some(a) => a,
        None => panic!("No solution found!"),
    }
}

fn find_a(current_a: i64, target_idx: usize, program: &Program) -> Option<i64> {
    for digit in 0..8 {
        // Shift current candidate left and try the next 3 bits
        let candidate_a = (current_a << 3) | digit;

        // Run the program with this candidate to see what it ouptuts
        let output = run_for_a(candidate_a, program);

        // We check if the FIRST output matches the program digit at target_idx
        if !output.is_empty() && output[0] == program.program[target_idx] {
            // Base case: we've matched the whole program (reached index 0)
            if target_idx == 0 {
                return Some(candidate_a);
            }

            // Recursive step: try to match the digit at target_idx - 1
            if let Some(res) = find_a(candidate_a, target_idx - 1, program) {
                return Some(res);
            }
        }
    }
    None
}

// Helper to run the program and get the first output digit
fn run_for_a(a: i64, program: &Program) -> Vec<u8> {
    let mut copy = program.clone();
    copy.register_a = a;
    copy.pointer = 0;

    let mut output = Vec::new();
    while copy.pointer < copy.instructions.len() {
        let op = copy.instructions[copy.pointer];
        let operand = copy.operands[copy.pointer] as i64;
        let result = op.apply(operand, &mut copy);

        if op == Operation::Out {
            output.push(result.unwrap() as u8);
            // Optimization: we usually only need the first digit to verify
            // the current 3-bit chunk
            return output;
        }
        if op == Operation::Jnz && result.is_some() {
            continue;
        }
        copy.pointer += 1;
    }
    output
}

#[cfg(test)]
mod day17 {
    use super::*;

    fn get_example1() -> Vec<String> {
        vec![
            String::from("Register A: 729"),
            String::from("Register B: 0"),
            String::from("Register C: 0"),
            String::from(""),
            String::from("Program: 0,1,5,4,3,0"),
        ]
    }

    fn get_example2() -> Vec<String> {
        vec![
            String::from("Register A: 2024"),
            String::from("Register B: 0"),
            String::from("Register C: 0"),
            String::from(""),
            String::from("Program: 0,3,5,4,3,0"),
        ]
    }

    #[test]
    fn day17_parse() {
        let program = parse_day17(&get_example1());
        assert_eq!(program.register_a, 729);
        assert_eq!(program.register_b, 0);
        assert_eq!(program.register_c, 0);
        assert_eq!(program.pointer, 0);
        assert_eq!(program.instructions.len(), 3);
        assert_eq!(program.operands.len(), 3);
        assert_eq!(
            program.instructions,
            vec![Operation::Adv, Operation::Out, Operation::Jnz]
        );
        assert_eq!(program.operands, vec![1, 4, 0]);
    }

    #[test]
    fn day17_single_operations() {
        // If register C contains 9, the program 2,6 would set register B to 1
        let mut program = Program {
            register_c: 9,
            ..Default::default()
        };
        let result = Operation::Bst.apply(6, &mut program);
        assert!(result.is_none());
        assert_eq!(program.register_b, 1);
        assert_eq!(program.register_c, 9);

        //If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
        program = Program {
            register_a: 10,
            ..Default::default()
        };
        let mut output = Vec::new();
        if let Some(result) = Operation::Out.apply(0, &mut program) {
            output.push(result);
        }
        if let Some(result) = Operation::Out.apply(1, &mut program) {
            output.push(result);
        }
        if let Some(result) = Operation::Out.apply(4, &mut program) {
            output.push(result);
        }
        assert_eq!(output, vec![0, 1, 2]);

        // If register B contains 29, the program 1,7 would set register B to 26.
        program = Program {
            register_b: 29,
            ..Default::default()
        };
        let result = Operation::Bxl.apply(7, &mut program);
        assert!(result.is_none());
        assert_eq!(program.register_b, 26);

        // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
        program = Program {
            register_b: 2024,
            register_c: 43690,
            ..Default::default()
        };
        let result = Operation::Bxc.apply(0, &mut program);
        assert!(result.is_none());
        assert_eq!(program.register_b, 44354);
        assert_eq!(program.register_c, 43690);
    }

    #[test]
    fn day17_loop_operation() {
        // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
        let mut program = Program {
            register_a: 2024,
            instructions: vec![Operation::Adv, Operation::Out, Operation::Jnz],
            operands: vec![1, 4, 0],
            ..Default::default()
        };
        let output = run_program_day17_stage1(&mut program);
        assert_eq!(program.register_a, 0);
        assert_eq!(output, String::from("4,2,5,6,7,7,7,7,3,1,0"));
    }

    #[test]
    fn day17_stage1_example() {
        let mut program = parse_day17(&get_example1());
        let output = run_program_day17_stage1(&mut program);
        assert_eq!(output, String::from("4,6,3,5,6,3,5,2,1,0"));
    }

    #[test]
    fn day17_stage2_check_result() {
        let mut program = parse_day17(&get_example2());
        program.register_a = 117440;
        let output = run_program_day17_stage1(&mut program);
        assert_eq!(output, String::from("0,3,5,4,3,0"));
    }

    #[test]
    fn day17_stage2_example() {
        let program = parse_day17(&get_example2());
        let reg_a = get_register_a_day17_stage2(&program);
        assert_eq!(reg_a, 117440);

        let mut copy = program.clone();
        copy.register_a = reg_a;
        copy.register_b = program.initial_b;
        copy.register_c = program.initial_c;
        copy.pointer = 0;

        let output = run_program_day17_stage1(&mut copy);
        assert_eq!(output, String::from("0,3,5,4,3,0"));
    }
}
