mod cpu;
mod instruction;

use std::fs;
use instruction::Instruction;


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

    let mut cpu = cpu::Cpu::new();
    let mut program = Program::new();

    program.parse(&contents);
    program.print();

    cpu.run(&program.instructions);

    cpu.print_gp_reg_vals();
}
