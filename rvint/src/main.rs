use std::fs;

pub struct Instruction {
    opcode: String,
    args: Vec<String>,
}

impl Instruction {

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
                args_str.push(s.to_string());
            }

            self.instructions.push(Instruction {
                opcode: opcode.to_string(),
                args: args_str,
            })
        }
    }
}


fn main() {
    let file_path = "test.rv";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut program = Program::new();

    program.parse(&contents);
    program.print();
}
