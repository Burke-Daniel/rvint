use std::{str::FromStr, fmt::Debug};

use crate::instruction::Instruction;

pub struct Cpu {
    pub pc: usize,
    pub gp_regs: [u32; 32],
    pub fp_regs: [f32; 32],
    pub memory: [u32; 4096],
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: 0,
            gp_regs: [0; 32],
            fp_regs: [0.0; 32],
            memory: [0; 4096],
        }
    }

    pub fn run(&mut self, instructions: &Vec<Instruction>) {
        while self.pc < instructions.len() {
            self.execute_instruction(&instructions[self.pc]);
        }
    }

    pub fn parse_three_args<Dest, A1, A2>(&self, instruction: &Instruction) -> (Dest, A1, A2)
    where Dest: FromStr + Debug, <Dest as FromStr>::Err : Debug,
        A1: FromStr + Debug, <A1 as FromStr>::Err : Debug,
        A2: FromStr + Debug, <A2 as FromStr>::Err : Debug,
    {
        assert!(instruction.args.len() == 3);
        let dest = instruction.args[0].parse::<Dest>().unwrap();
        let a1 = instruction.args[1].parse::<A1>().unwrap();
        let a2 = instruction.args[2].parse::<A2>().unwrap();

        return (dest, a1, a2);
    }

    pub fn parse_two_args<Dest, A1>(&self, instruction: &Instruction) -> (Dest, A1)
    where Dest: FromStr + Debug, <Dest as FromStr>::Err : Debug,
        A1: FromStr + Debug, <A1 as FromStr>::Err : Debug,
    {
        assert!(instruction.args.len() == 3);
        let dest = instruction.args[0].parse::<Dest>().unwrap();
        let a1 = instruction.args[1].parse::<A1>().unwrap();

        return (dest, a1);
    }

    pub fn execute_instruction(&mut self, instruction: &Instruction) {
        let arithmetic_instructions = [
            "add",
            "sub",
            "mul",
            "div",
            "and",
            "or",
            "xor",
            "sll",
            "srl",
            "sra",
            "slt",
            "sltu",
            "lui",
        ];

        let immediate_arithmetic_instructions = [
            "addi",
            "subi",
            "andi",
            "ori",
            "xori",
            "slli",
            "srli",
            "srai",
        ];
        
        if arithmetic_instructions.contains(&instruction.opcode.to_lowercase().as_str()) {
            self.handle_arithmetic_instruction(instruction);
        } else if immediate_arithmetic_instructions.contains(&instruction.opcode.to_lowercase().as_str()) {
            self.handle_immediate_arithmetic_instruction(instruction);
        }
    }

    fn perform_arithmetic_operation(&mut self, opcode: &String, dest: usize, a1: usize, a2: usize) {
        match opcode.to_lowercase().as_str() {
            "add" => {
                self.gp_regs[dest] = self.gp_regs[a1] + self.gp_regs[a2];
            }
            "sub" => {
                self.gp_regs[dest] = self.gp_regs[a1] - self.gp_regs[a2];
            }
            "mul" => {
                self.gp_regs[dest] = self.gp_regs[a1] * self.gp_regs[a2];
            }
            "div" => {
                self.gp_regs[dest] = self.gp_regs[a1] / self.gp_regs[a2];
            }
            "and" => {
                self.gp_regs[dest] = self.gp_regs[a1] & self.gp_regs[a2];
            }
            "or" => {
                self.gp_regs[dest] = self.gp_regs[a1] | self.gp_regs[a2];
            }
            "xor" => {
                self.gp_regs[dest] = self.gp_regs[a1] ^ self.gp_regs[a2];
            }
            "slt" => {
                self.gp_regs[dest] = if self.gp_regs[a1] < self.gp_regs[a2] { 1 } else { 0 };
            }
            "sltu" => {
                // TODO differentiate between signed and unsigned
                self.gp_regs[dest] = if self.gp_regs[a1] < self.gp_regs[a2] { 1 } else { 0 };
            }
            "sll" => {
                self.gp_regs[dest] = self.gp_regs[a1] << self.gp_regs[a2];
            }
            "srl" => {
                self.gp_regs[dest] = self.gp_regs[a1] >> self.gp_regs[a2];
            }
            "sra" => {
                let msb: u32 = self.gp_regs[a1] & (1 << 31);
                self.gp_regs[dest] = (self.gp_regs[a1] >> self.gp_regs[a2]) & msb;
            }
            _ => todo!()
        }
    }

    fn perform_immediate_arithmetic_operation(&mut self, opcode: &String, dest: usize, a1: usize, a2: u32) {
        match opcode.to_lowercase().as_str() {
            "addi" => {
                self.gp_regs[dest] = self.gp_regs[a1] + a2;
            }
            "subi" => {
                self.gp_regs[dest] = self.gp_regs[a1] - a2;
            }
            "andi" => {
                self.gp_regs[dest] = self.gp_regs[a1] & a2;
            }
            "ori" => {
                self.gp_regs[dest] = self.gp_regs[a1] | a2;
            }
            "xori" => {
                self.gp_regs[dest] = self.gp_regs[a1] ^ a2;
            }
            "slli" => {
                self.gp_regs[dest] = self.gp_regs[a1] << a2;
            }
            "srli" => {
                self.gp_regs[dest] = self.gp_regs[a1] >> a2;
            }
            "srai" => {
                // todo test that this is correct
                let msb: u32 = self.gp_regs[a1] & (1 << 31);
                self.gp_regs[dest] = (self.gp_regs[a1] >> a2) & msb;
            }
            _ => todo!()
        }
    }

    fn handle_arithmetic_instruction(&mut self, instruction: &Instruction) {
        println!("{} instruction!", instruction.opcode);
        if instruction.opcode != "lui" {
            assert!(instruction.args.len() == 3);
            let (dest, a1, a2): (usize, usize, usize) = self.parse_three_args(instruction);
            assert!(dest <= 32);

            self.perform_arithmetic_operation(&instruction.opcode, dest, a1, a2);
            self.pc += 1;
        } else {
            assert!(instruction.args.len() == 2);
            let (dest, a1): (usize, u32) = self.parse_two_args(instruction);
            assert!(dest <= 32);

            self.gp_regs[dest] = a1 << 12;
        }
    }

    fn handle_immediate_arithmetic_instruction(&mut self, instruction: &Instruction) {
        println!("{} instruction!", instruction.opcode);
        assert!(instruction.args.len() == 3);
        let (dest, a1, a2): (usize, usize, u32) = self.parse_three_args(instruction);
        assert!(dest <= 32);

        self.perform_immediate_arithmetic_operation(&instruction.opcode, dest, a1, a2);
        self.pc += 1;
    }

    pub fn print_gp_reg_vals(&self) {
        println!("GP Reg values:");
        for (reg_num, reg_val) in self.gp_regs.iter().enumerate() {
            println!("{reg_num}: {reg_val}");
        }
    }

    pub fn print_fp_reg_vals(&self) {
        println!("FP Reg values:");
        for (reg_num, reg_val) in self.fp_regs.iter().enumerate() {
            println!("{reg_num}: {reg_val}");
        }
    }
}

#[test]
fn test_add_instruction() {
    let mut cpu = Cpu::new();

    cpu.gp_regs[1] = 2;
    cpu.gp_regs[2] = 3;

    let instruction = Instruction {
        opcode: "add".to_string(),
        args: vec![ "0".to_string(), "1".to_string(), "2".to_string() ],
    };

    cpu.execute_instruction(&instruction);

    assert!(cpu.gp_regs[0] == 5);
}

#[test]
fn test_sub_instruction() {
    let mut cpu = Cpu::new();

    cpu.gp_regs[1] = 3;
    cpu.gp_regs[2] = 2;

    let instruction = Instruction {
        opcode: "sub".to_string(),
        args: vec![ "0".to_string(), "1".to_string(), "2".to_string() ],
    };

    cpu.execute_instruction(&instruction);

    assert!(cpu.gp_regs[0] == 1);
}

#[test]
fn test_mul_instruction() {
    let mut cpu = Cpu::new();

    cpu.gp_regs[1] = 3;
    cpu.gp_regs[2] = 2;

    let instruction = Instruction {
        opcode: "mul".to_string(),
        args: vec![ "0".to_string(), "1".to_string(), "2".to_string() ],
    };

    cpu.execute_instruction(&instruction);

    assert!(cpu.gp_regs[0] == 6);
}

#[test]
fn test_div_instruction() {
    let mut cpu = Cpu::new();

    cpu.gp_regs[1] = 3;
    cpu.gp_regs[2] = 2;

    let instruction = Instruction {
        opcode: "div".to_string(),
        args: vec![ "0".to_string(), "1".to_string(), "2".to_string() ],
    };

    cpu.execute_instruction(&instruction);

    assert!(cpu.gp_regs[0] == 1);
}
