mod cpu;
mod instruction;
mod parser;

use std::fs;
use instruction::Instruction;
use parser::Parser;


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
}


fn main() {
    let file_path = "test.rv";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut cpu = cpu::Cpu::new();
    let mut program = Program::new();

    let mut parser = Parser::new(contents);
    parser.parse();

    program.print();

    cpu.run(&program.instructions);

    cpu.print_gp_reg_vals();
}
