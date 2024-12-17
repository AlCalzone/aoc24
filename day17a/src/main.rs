use std::time::Instant;

const INPUT: &'static str = include_str!("input.txt");

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

    let lines = INPUT.lines().collect::<Vec<_>>();
    let reg_a = lines[0].split_once(": ").unwrap().1.parse().unwrap();
    let reg_b = lines[1].split_once(": ").unwrap().1.parse().unwrap();
    let reg_c = lines[2].split_once(": ").unwrap().1.parse().unwrap();

    let instr_raw: Vec<_> = lines[4]
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let instructions: Vec<_> = instr_raw
        .chunks_exact(2)
        .map(|x| Instruction::from([x[0], x[1]]))
        .collect();

    // println!("Instructions:");
    // for instr in &instructions {
    //     println!("{:?}", instr);
    // }

    let mut computer = Computer {
        registers: [reg_a, reg_b, reg_c],
        program: instructions,
        pc: 0,
        output: Vec::new(),
    };

    computer.run(false);

    let result = computer
        .output
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join(",");

    let elapsed = start.elapsed();

    println!("Register A: {}", computer.get_register(Register::A));
    println!("Register B: {}", computer.get_register(Register::B));
    println!("Register C: {}", computer.get_register(Register::C));
    println!("Output: {}", result);
    println!();
    println!("(took: {:?})", elapsed);
}
