pub struct Instruction {
    pub opcode: String,
    pub args: Vec<String>,
}

impl Instruction {
    pub fn new(opcode: &String, args: &Vec<String>) -> Self {
        Self {
            opcode: opcode.to_string(),
            args: args.to_vec(),
        }
    }
}