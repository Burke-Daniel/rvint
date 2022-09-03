use std::fs;

pub struct Cpu {
    pc: usize,
    gp_regs: [u32; 32],
    fp_regs: [f32; 32],
    memory: [u32; 4096],
}

impl Cpu {
    fn new() -> Self {
        Self {
            pc: 0,
            gp_regs: [0; 32],
            fp_regs: [0.0; 32],
            memory: [0; 4096],
        }
    }

    fn run(&mut self, instructions: &Vec<Instruction>) {
        self.execute_instruction(&instructions[self.pc]);
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction.opcode.to_lowercase().as_str() {
            "add" => {
                println!("Add instruction!");
                assert!(instruction.args.len() == 3);
                let dest = instruction.args[0].parse::<usize>().unwrap();
                let a1 = instruction.args[1].parse::<usize>().unwrap();
                let a2 = instruction.args[1].parse::<usize>().unwrap();

                assert!(dest <= 32);
                self.gp_regs[dest] = self.gp_regs[a1] + self.gp_regs[a2];
            }
            _ => { println!("Invalid Instruction. Skipping..."); }
        }
    }

    fn print_gp_reg_vals(&self) {
        println!("GP Reg values:");
        for (reg_num, reg_val) in self.gp_regs.iter().enumerate() {
            println!("{reg_num}: {reg_val}");
        }
    }

    fn print_fp_reg_vals(&self) {
        println!("FP Reg values:");
        for (reg_num, reg_val) in self.fp_regs.iter().enumerate() {
            println!("{reg_num}: {reg_val}");
        }
    }
}

pub struct Instruction {
    opcode: String,
    args: Vec<String>,
}

impl Instruction {
    fn new(opcode: &String, args: &Vec<String>) -> Self {
        Self {
            opcode: opcode.to_string(),
            args: args.to_vec(),
        }
    }
}

pub struct Program {
    instructions: Vec<Instruction>
}

impl Program {
    fn new() -> Self {
        Self { instructions: Vec::new() }
    }

    fn print(self: &Self) {
        for (i, instruction) in self.instructions.iter().enumerate() {
            println!("Instruction {}:", i);
            println!("\tOpcode: {}", instruction.opcode);
            println!("\tArgs: {}", instruction.args.join(" "));
        }
    }

    fn parse(self: &mut Self, program: &str) {
        let lines = program.lines();

        for line in lines {
            let mut tokens = line.split_whitespace();
            let opcode = tokens.next().unwrap();

            let args_raw: Vec<&str> = tokens.collect();
            let mut args_str: Vec<String> = Vec::new();
            for s in &args_raw {
                let current_arg = s.strip_suffix(",");
                if current_arg == None {
                    args_str.push(s.to_string());
                }
                else {
                    args_str.push(current_arg.unwrap().to_string());
                }
            }

            self.instructions.push(Instruction::new(&opcode.to_string(), &args_str));
        }
    }
}


fn main() {
    let file_path = "test.rv";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut cpu = Cpu::new();
    let mut program = Program::new();

    program.parse(&contents);
    program.print();

    cpu.print_gp_reg_vals();
    cpu.print_fp_reg_vals();
    cpu.run(&program.instructions);
    cpu.print_gp_reg_vals();
    cpu.print_fp_reg_vals();
}
