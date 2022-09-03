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

    pub fn parse_args_arithmetic(&self, instruction: &Instruction) -> (usize, usize, usize) {
        assert!(instruction.args.len() == 3);
        let dest = instruction.args[0].parse::<usize>().unwrap();
        let a1 = instruction.args[1].parse::<usize>().unwrap();
        let a2 = instruction.args[2].parse::<usize>().unwrap();

        return (dest, a1, a2);
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
        ];
        
        if arithmetic_instructions.contains(&instruction.opcode.to_lowercase().as_str()) {
            self.handle_arithmetic_instruction(instruction);
        }
    }

    fn handle_arithmetic_instruction(&mut self, instruction: &Instruction) {
        match instruction.opcode.to_lowercase().as_str() {
            "add" => {
                println!("Add instruction!");
                assert!(instruction.args.len() == 3);
                let (dest, a1, a2) = self.parse_args_arithmetic(instruction);

                assert!(dest <= 32);
                self.gp_regs[dest] = self.gp_regs[a1] + self.gp_regs[a2];
                self.pc += 1;
            }
            "sub" => {
                println!("Sub instruction!");
                assert!(instruction.args.len() == 3);
                let (dest, a1, a2) = self.parse_args_arithmetic(instruction);

                assert!(dest <= 32);
                self.gp_regs[dest] = self.gp_regs[a1] - self.gp_regs[a2];
                self.pc += 1;
            }
            "mul" => {
                println!("Mul instruction!");
                assert!(instruction.args.len() == 3);
                let (dest, a1, a2) = self.parse_args_arithmetic(instruction);

                assert!(dest <= 32);
                self.gp_regs[dest] = self.gp_regs[a1] * self.gp_regs[a2];
                self.pc += 1;
            }
            "div" => {
                println!("Div instruction!");
                assert!(instruction.args.len() == 3);
                let (dest, a1, a2) = self.parse_args_arithmetic(instruction);

                assert!(dest <= 32 && self.gp_regs[a2] != 0);
                self.gp_regs[dest] = self.gp_regs[a1] / self.gp_regs[a2];
                self.pc += 1;
            }
            "and" => {
                println!("And instruction!");
                assert!(instruction.args.len() == 3);
                let (dest, a1, a2) = self.parse_args_arithmetic(instruction);

                assert!(dest <= 32);
                self.gp_regs[dest] = self.gp_regs[a1] & self.gp_regs[a2];
                self.pc += 1;
            }
            "or" => {
                println!("Or instruction!");
                assert!(instruction.args.len() == 3);
                let (dest, a1, a2) = self.parse_args_arithmetic(instruction);

                assert!(dest <= 32);
                self.gp_regs[dest] = self.gp_regs[a1] | self.gp_regs[a2];
                self.pc += 1;
            }
            "xor" => {
                println!("Xor instruction!");
                assert!(instruction.args.len() == 3);
                let (dest, a1, a2) = self.parse_args_arithmetic(instruction);

                assert!(dest <= 32);
                self.gp_regs[dest] = self.gp_regs[a1] ^ self.gp_regs[a2];
                self.pc += 1;
            }
            _ => {
                println!("Invalid Instruction. Skipping...");
                
                // TODO change
                // Not very clean way to terminate program cleanly on invalid instruction
                self.pc = usize::MAX;
            }
        }
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
