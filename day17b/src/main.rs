use core::panic;
use std::time::Instant;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum Register {
    A,
    B,
    C,
}

type Literal = u8;

#[derive(Clone, Copy, Debug)]
enum ComboOp {
    Literal(Literal),
    Register(Register),
}

impl From<u8> for ComboOp {
    fn from(value: u8) -> Self {
        match value {
            0..=3 => ComboOp::Literal(value),
            4 => ComboOp::Register(Register::A),
            5 => ComboOp::Register(Register::B),
            6 => ComboOp::Register(Register::C),
            _ => panic!("Invalid value for ComboOp: {}", value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Adv(ComboOp),
    Bxl(Literal),
    Bst(ComboOp),
    Jnz(Literal),
    Bxc,
    Out(ComboOp),
    Bdv(ComboOp),
    Cdv(ComboOp),
}

impl From<[u8; 2]> for Instruction {
    fn from(value: [u8; 2]) -> Self {
        match value {
            [0, a] => Instruction::Adv(a.into()),
            [1, a] => Instruction::Bxl(a),
            [2, a] => Instruction::Bst(a.into()),
            [3, a] => Instruction::Jnz(a),
            [4, _] => Instruction::Bxc,
            [5, a] => Instruction::Out(a.into()),
            [6, a] => Instruction::Bdv(a.into()),
            [7, a] => Instruction::Cdv(a.into()),
            _ => panic!("Invalid value for Instruction: {:?}", value),
        }
    }
}

struct Computer {
    registers: [usize; 3],
    program: Vec<Instruction>,
    pc: usize,
    output: Vec<u8>,
}

impl Computer {
    fn get_register(&self, reg: Register) -> usize {
        self.registers[reg as usize]
    }

    fn set_register(&mut self, reg: Register, value: usize) {
        self.registers[reg as usize] = value;
    }

    fn get_value(&self, op: ComboOp) -> usize {
        match op {
            ComboOp::Literal(value) => value as usize,
            ComboOp::Register(reg) => self.get_register(reg),
        }
    }

    pub fn run(&mut self, debug: bool) {
        if debug {
            println!("PC |        A |        B |        C | Instr");
        }
        while self.pc < self.program.len() * 2 {
            let instr = self.program[self.pc / 2];
            if debug {
                print!(
                    "{:02} | {:08x} | {:08x} | {:08x} | ",
                    self.pc, self.registers[0], self.registers[1], self.registers[2]
                );
            }
            match instr {
                Instruction::Bxl(op) => {
                    let reg = self.get_register(Register::B);
                    if debug {
                        println!("Bxl({}) -> {} ^ {}", op, reg, op);
                    }
                    let result = reg ^ (op as usize);
                    self.set_register(Register::B, result);
                }

                Instruction::Bst(op) => {
                    let value = self.get_value(op);
                    if debug {
                        println!("Bst({:?}) -> {}", op, value);
                    }
                    self.set_register(Register::B, value & 0b111);
                }

                Instruction::Jnz(op) => {
                    let condition = self.get_register(Register::A);
                    if debug {
                        println!("Jnz({}) -> {}", op, condition);
                    }
                    if condition != 0 {
                        self.pc = op as usize;
                        continue;
                    }
                }

                Instruction::Bxc => {
                    let reg_b = self.get_register(Register::B);
                    let reg_c = self.get_register(Register::C);
                    if debug {
                        println!("Bxc -> {} ^ {}", reg_b, reg_c);
                    }
                    let result = reg_b ^ reg_c;
                    self.set_register(Register::B, result);
                }

                Instruction::Out(op) => {
                    let value = self.get_value(op);
                    if debug {
                        println!("Out({:?}) -> {}", op, value);
                    }
                    self.output.push((value & 0b111) as u8);
                }

                Instruction::Adv(op) => {
                    let amount = self.get_value(op);
                    if debug {
                        println!("Adv({:?}) -> A >> {}", op, amount);
                    }
                    let numerator = self.get_register(Register::A);
                    let result = numerator >> amount;
                    self.set_register(Register::A, result);
                }

                Instruction::Bdv(op) => {
                    let amount = self.get_value(op);
                    if debug {
                        println!("Bdv({:?}) -> B >> {}", op, amount);
                    }
                    let numerator = self.get_register(Register::A);
                    let result = numerator >> amount;
                    self.set_register(Register::B, result);
                }

                Instruction::Cdv(op) => {
                    let amount = self.get_value(op);
                    if debug {
                        println!("Cdv({:?}) -> C >> {}", op, amount);
                    }
                    let numerator = self.get_register(Register::A);
                    let result = numerator >> amount;
                    self.set_register(Register::C, result);
                }
            }

            self.pc += 2;
        }
    }
}

fn main() {
    let start = Instant::now();

    let instr_raw: Vec<u8> = vec![2, 4, 1, 2, 7, 5, 4, 7, 1, 3, 5, 5, 0, 3, 3, 0];

    let instructions: Vec<_> = instr_raw
        .chunks_exact(2)
        .map(|chunk| Instruction::from([chunk[0], chunk[1]]))
        .collect();

    // Observation:
    // - Register A must be a number with the same amount of octal digits as the number of bytes in the program.
    // - The last printed number corresponds to the most significant digit of the number in register A.
    // - The mapping between digits and output seems not to be trivial, so we have to try multiple ones
    // - The mapping may affect up to 1 other digits, so we need to do some backtracking
    // - The most significant digit may not be 0

    let mut result: Vec<u8> = vec![0; instr_raw.len()];

    for digit in 0..result.len() {
        if !solve_digit(&mut result, &instr_raw, &instructions, digit, 1) {
            panic!("Failed to find solution for digit {}", digit);
        }
    }

    let elapsed = start.elapsed();

    // Sanity check:
    let input = result.iter().fold(0, |acc, &x| acc * 8 + (x as usize));

    let mut computer = Computer {
        registers: [input, 0, 0],
        program: instructions.clone(),
        pc: 0,
        output: Vec::new(),
    };

    computer.run(false);

    assert_eq!(computer.output, instr_raw);

    println!("Solution: {:?}", input);
    println!("Output:   {:?}", computer.output);
    println!("Expected: {:?}", instr_raw);
    println!();
    println!("(took: {:?})", elapsed);
}

fn solve_digit(
    result: &mut [u8],
    instr_raw: &[u8],
    instructions: &Vec<Instruction>,
    digit: usize,
    depth: usize,
) -> bool {
    for value in 0..8 {
        if value == 0 && digit == 0 {
            continue;
        }
        result[digit] = value;

        // Reset digits that were changed during a previous lookahead
        for i in digit + 1..result.len() {
            result[i] = 0;
        }

        let input = result.iter().fold(0, |acc, &x| acc * 8 + (x as usize));

        let mut computer = Computer {
            registers: [input, 0, 0],
            program: instructions.clone(),
            pc: 0,
            output: Vec::new(),
        };

        computer.run(false);

        assert_eq!(computer.output.len(), instr_raw.len());

        if computer.output[computer.output.len() - digit - 1]
            == instr_raw[computer.output.len() - digit - 1]
        {
            // This is a possible solution. Try solving the next digits.
            // If those succeed as well, consider this the solution.
            if depth == 0
                || digit == result.len() - 1
                || solve_digit(result, instr_raw, instructions, digit + 1, depth - 1)
            {
                return true;
            }
        }
    }

    false
}
