use crate::instruction::Instruction;

pub fn parse(program: &str) -> Vec<Instruction> {
    let lines = program.lines();
    let mut instructions: Vec<Instruction> = vec![];

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

        instructions.push(Instruction::new(&opcode.to_string(), &args_str));
    }

    return instructions;
}