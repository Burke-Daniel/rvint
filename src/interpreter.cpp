#include "interpreter.h"
#include <string>

void Interpreter::load_upper_immediate(int reg, int imm)
{
    gen_regs[reg] = imm << 20;
}

void Interpreter::interpret(const Instruction& instruction)
{
    if (instruction.instruction == "lui")
    {
        uint32_t reg = std::stod(instruction.operands[0]);
        uint32_t imm = std::stod(instruction.operands[1]);

        load_upper_immediate(reg, imm);
    }
    else if (instruction.instruction == "auipc")
    {}
    else if (instruction.instruction == "jal")
    {}
    else if (instruction.instruction == "jalr")
    {}
    else if (instruction.instruction == "beq")
    {}
    else if (instruction.instruction == "bne")
    {}
    else if (instruction.instruction == "blt")
    {}
    else if (instruction.instruction == "bge")
    {}
    else if (instruction.instruction == "bltu")
    {}
    else if (instruction.instruction == "bgeu")
    {}
    else if (instruction.instruction == "lb")
    {}
    else if (instruction.instruction == "lh")
    {}
    else if (instruction.instruction == "lw")
    {}
    else if (instruction.instruction == "lbu")
    {}
    else if (instruction.instruction == "lhu")
    {}
    else if (instruction.instruction == "sb")
    {}
    else if (instruction.instruction == "sh")
    {}
    else if (instruction.instruction == "sw")
    {}
    else if (instruction.instruction == "addi")
    {}
    else if (instruction.instruction == "slti")
    {}
    else if (instruction.instruction == "sltiu")
    {}
    else if (instruction.instruction == "xori")
    {}
    else if (instruction.instruction == "ori")
    {}
    else if (instruction.instruction == "andi")
    {}
    else if (instruction.instruction == "slli")
    {}
    else if (instruction.instruction == "srli")
    {}
    else if (instruction.instruction == "srai")
    {}
    else if (instruction.instruction == "add")
    {}
    else if (instruction.instruction == "sll")
    {}
    else if (instruction.instruction == "slt")
    {}
    else if (instruction.instruction == "sltu")
    {}
    else if (instruction.instruction == "xor")
    {}
    else if (instruction.instruction == "srl")
    {}
    else if (instruction.instruction == "sra")
    {}
    else if (instruction.instruction == "or")
    {}
    else if (instruction.instruction == "and")
    {}
}