""" rvasm - a Risc-V (maybe not)assembler/interpreter written in python"""


""" Ideas for extension once core interpreter is complete
- Command line debugger
- Interactive repl mode
- Compile to either machine code or some form of bytecode
- Add macros for useful stuff to interpreter (similar to C)
"""

""" TODOs
- TODO implement interpret for store and branch ops
- TODO properly parse branch syntax (with arbitrary tags, etc.)
- TODO properly unit test each instruction
- TODO argument parsing with argparse if necessary
- TODO try reading in input from file
"""

import re

# Constants

# Change this to increase the amount of memory the interpreter has to work with
MEMORY_SIZE = 4096

# Globals

# Program counter
PC = 0

# General Purpose registers, store integers and addresses
GP_REGS = [0] * 32

# Floating Point registers, used for floating point ops
FP_REGS = [0.0] * 32

# Memory - technically infinite
MEM = bytearray(MEMORY_SIZE)

# Instruction categories

BASE_ARITHMETIC_OPS = [
    "add",
    "sub",
    "xor",
    "or",
    "and",
    "sll",
    "srl",
    "sra",
    "slt",
    "sltu"
]

BASE_IMM_ARITHMETIC_OPS = [
    "addi",
    "subi",
    "xori",
    "ori",
    "andi",
    "slli",
    "srli",
    "srai",
    "slti",
    "sltiu"
]

LOAD_OPS = [
    "lb",
    "lh",
    "lw",
    "lbu",
    "lhu"
]

STORE_OPS = [
    "sb",
    "sh",
    "sw"
]

BRANCH_OPS = [
    "beq",
    "bne",
    "blt",
    "bge",
    "bltu",
    "bgeu"
]

# Helpers

def convert_to_index(register) -> int:
    """ Converts from regval in string to integer"""
    return int(register[1:])

def reset_interpreter():
    global PC, GP_REGS, FP_REGS, MEM
    PC = 0
    GP_REGS = [0] * 32
    FP_REGS = [0.0] * 32
    MEM = bytearray(MEMORY_SIZE)


# Parser functions

def parse_input(input: str):
    """ Splits input by newline characters"""
    parsed_input = []
    if '\r' in input:
        parsed_input = input.split("\r\n")
    else:
        parsed_input = input.split("\n")
    return list(filter(lambda instruction: instruction != "", parsed_input))

def convert_token_to_int(token):
    """ Try and convert a token to an integer"""
    try:
        return int(token)
    except ValueError:
        return token

def tokenize_instruction(instruction: str):
    # Tokenize based off of commas and spaces
    return list(map(convert_token_to_int, re.split(' |, |,', instruction)))

def interpret(instruction_list, num_instructions):
    global PC
    while PC < num_instructions:
        tokens = tokenize_instruction(instruction_list[PC])
        if tokens[0] in BASE_ARITHMETIC_OPS:
            des_reg = convert_to_index(tokens[1])
            arg_reg1 = convert_to_index(tokens[2])
            arg_reg2 = convert_to_index(tokens[3])
            base_arithmetic_instruction(tokens[0], (des_reg, arg_reg1, arg_reg2))
            PC += 1
        elif tokens[0] in BASE_IMM_ARITHMETIC_OPS:
            des_reg = convert_to_index(tokens[1])
            arg_reg = convert_to_index(tokens[2])
            arg_imm = tokens[3]
            base_imm_arithmetic_instruction(tokens[0], (des_reg, arg_reg, arg_imm))
            PC += 1
        elif tokens[0] in LOAD_OPS:
            des_reg = convert_to_index(tokens[1])
            mem = list(map(lambda str: str.strip(')'), tokens[2].split('(')))
            arg_imm = int(mem[0])
            arg_reg = convert_to_index(mem[1])
            load_instructions(tokens[0], (des_reg, arg_reg, arg_imm))
            PC += 1
        elif tokens[0] in STORE_OPS:
            # TODO
            pass
        elif tokens[0] in BRANCH_OPS:
            # TODO
            pass
        # r0 should always be zero, regardless of what is copied into it
        GP_REGS[0] = 0


# Core instruction groups

def base_arithmetic_instruction(instruction, args: tuple):
    des, reg1, reg2 = args
    assert(abs(des) < 32 and abs(reg1) < 32 and abs(reg2) < 32)

    if instruction == "add":
        instruction_add(des, reg1, reg2)
    elif instruction == "sub":
        instruction_sub(des, reg1, reg2)
    elif instruction == "xor":
        instruction_xor(des, reg1, reg2)
    elif instruction == "or":
        instruction_or(des, reg1, reg2)
    elif instruction == "and":
        instruction_and(des, reg1, reg2)
    elif instruction == "sub":
        instruction_sub(des, reg1, reg2)
    elif instruction == "srl":
        instruction_srl(des, reg1, reg2)
    elif instruction == "sra":
        instruction_sra(des, reg1, reg2)
    elif instruction == "slt":
        instruction_slt(des, reg1, reg2)
    elif instruction == "sltu":
        instruction_sltu(des, reg1, reg2)

def base_imm_arithmetic_instruction(instruction, args: tuple):
    des, reg, imm = args
    assert(abs(des) < 32 and abs(reg) < 32)

    if instruction == "addi":
        instruction_addi(des, reg, imm)
    elif instruction == "xori":
        instruction_xori(des, reg, imm)
    elif instruction == "ori":
        instruction_ori(des, reg, imm)
    elif instruction == "andi":
        instruction_andi(des, reg, imm)
    elif instruction == "srli":
        instruction_srli(des, reg, imm)
    elif instruction == "srai":
        instruction_srai(des, reg, imm)
    elif instruction == "slti":
        instruction_slti(des, reg, imm)
    elif instruction == "sltiu":
        instruction_sltiu(des, reg, imm)

def load_instructions(instruction, args: tuple):
    des, reg, imm = args
    assert(abs(des) < 32 and abs(reg) < 32)
    if instruction == "lb":
        instruction_lb(des, reg, imm)
    elif instruction == "lh":
        instruction_lh(des, reg, imm)
    elif instruction == "lw":
        instruction_lw(des, reg, imm)
    elif instruction == "lbu":
        instruction_lbu(des, reg, imm)
    elif instruction == "lhu":
        instruction_lhu(des, reg, imm)

def store_instructions(instruction, args: tuple):
    des, reg, imm = args
    assert(abs(des) < 32 and abs(reg) < 32)
    if instruction == "sb":
        instruction_sb(des, reg, imm)
    elif instruction == "sh":
        instruction_sh(des, reg, imm)
    elif instruction == "sw":
        instruction_sw(des, reg, imm)

def branch_instructions(instruction, args: tuple):
    pass

# Base arithmetic instructions

def instruction_add(des: int, reg1: int, reg2: int):
    GP_REGS[des] = GP_REGS[reg1] + GP_REGS[reg2]

def instruction_sub(des: int, reg1: int, reg2: int):
    GP_REGS[des] = GP_REGS[reg1] - GP_REGS[reg2]

def instruction_xor(des: int, reg1: int, reg2: int):
    GP_REGS[des] = GP_REGS[reg1] ^ GP_REGS[reg2]

def instruction_or(des: int, reg1: int, reg2: int):
    GP_REGS[des] = GP_REGS[reg1] | GP_REGS[reg2]

def instruction_and(des: int, reg1: int, reg2: int):
    GP_REGS[des] = GP_REGS[reg1] & GP_REGS[reg2]

def instruction_sll(des: int, reg1: int, reg2: int):
    GP_REGS[des] = GP_REGS[reg1] << GP_REGS[reg2]

def instruction_srl(des: int, reg1: int, reg2: int):
    GP_REGS[des] = GP_REGS[reg1] >> GP_REGS[reg2]

def instruction_sra(des: int, reg1: int, reg2: int):
    # TODO figure out about arithmetic and logical shifts in python
    GP_REGS[des] = GP_REGS[reg1] >> GP_REGS[reg2]

def instruction_slt(des: int, reg1: int, reg2: int):
    GP_REGS[des] = 1 if GP_REGS[reg1] < GP_REGS[reg2] else 0

def instruction_sltu(des: int, reg1: int, reg2: int):
    # TODO figure out distinguishment between slt and sltu
    GP_REGS[des] = 1 if GP_REGS[reg1] < GP_REGS[reg2] else 0

# Base arithmetic instruction tests

def instruction_add_test() -> bool:
    # Load some values into registers
    GP_REGS[0] = 5
    GP_REGS[1] = 10

    instruction_add(0, 0, 1)
    assert(GP_REGS[0] == 15)

    GP_REGS[0] = 0
    GP_REGS[1] = 0
    return True

def instruction_sub_test() -> bool:
    # Load some values into registers
    GP_REGS[0] = 5
    GP_REGS[1] = 10

    instruction_sub(0, 0, 1)
    assert(GP_REGS[0] == -5)

    GP_REGS[0] = 0
    GP_REGS[1] = 0
    return True

def instruction_xor_test() -> bool:
    GP_REGS[0] = 0b01011010
    GP_REGS[1] = 0b11111111

    instruction_xor(0, 0, 1)
    assert(GP_REGS[0] == 0b10100101)

    GP_REGS[0] = 0
    GP_REGS[1] = 0
    return True

def instruction_or_test() -> bool:
    GP_REGS[0] = 0b01011010
    GP_REGS[1] = 0b11111111

    instruction_or(0, 0, 1)
    assert(GP_REGS[0] == 0b11111111)

    GP_REGS[0] = 0
    GP_REGS[1] = 0
    return True

def instruction_and_test() -> bool:
    GP_REGS[0] = 0b01011010
    GP_REGS[1] = 0b11111111

    instruction_and(0, 0, 1)
    assert(GP_REGS[0] == 0b01011010)

    GP_REGS[0] = 0
    GP_REGS[1] = 0
    return True

def instruction_sll_test() -> bool:
    GP_REGS[0] = 0b01011010
    GP_REGS[1] = 0b00000001

    instruction_sll(0, 0, 1)
    assert(GP_REGS[0] == 0b10110100)

    GP_REGS[0] = 0
    GP_REGS[1] = 0
    return True

def instruction_srl_test() -> bool:
    GP_REGS[0] = 0b01011010
    GP_REGS[1] = 0b00000001

    instruction_srl(0, 0, 1)
    assert(GP_REGS[0] == 0b00101101)

    GP_REGS[0] = 0
    GP_REGS[1] = 0
    return True

def instruction_sra_test() -> bool:
    # TODO
    assert(False)
    return True

def instruction_slt_test() -> bool:
    # TODO
    assert(False)
    return True

def instruction_sltu_test() -> bool:
    # TODO
    assert(False)
    return True

# Immediate base arithmetic instructions

def instruction_addi(des: int, reg: int, imm: int):
    GP_REGS[des] = GP_REGS[reg] + imm

def instruction_xori(des: int, reg: int, imm: int):
    GP_REGS[des] = GP_REGS[reg] ^ imm

def instruction_ori(des: int, reg: int, imm: int):
    GP_REGS[des] = GP_REGS[reg] | imm

def instruction_andi(des: int, reg: int, imm: int):
    GP_REGS[des] = GP_REGS[reg] & imm

def instruction_slli(des: int, reg: int, imm: int):
    GP_REGS[des] = GP_REGS[reg] << imm

def instruction_srli(des: int, reg: int, imm: int):
    GP_REGS[des] = GP_REGS[reg] >> imm

def instruction_srai(des: int, reg: int, imm: int):
    # TODO figure out the difference here
    GP_REGS[des] = GP_REGS[reg] >> imm

def instruction_slti(des: int, reg: int, imm: int):
    GP_REGS[des] = 1 if GP_REGS[reg] < imm else 0

def instruction_sltiu(des: int, reg: int, imm: int):
    # TODO figure out the difference here
    GP_REGS[des] = 1 if GP_REGS[reg] < imm else 0

# Immediate base arithmetic instruction tests

def instruction_addi_test() -> bool:
    # Load some values into registers
    GP_REGS[0] = 5
    imm = 10

    instruction_addi(0, 0, imm)
    assert(GP_REGS[0] == 15)

    GP_REGS[0] = 0
    return True

def instruction_xori_test():
    # TODO
    assert(False)
    return True

def instruction_ori_test():
    # TODO
    assert(False)
    return True

def instruction_andi_test():
    # TODO
    assert(False)
    return True

def instruction_slli_test():
    # TODO
    assert(False)
    return True

def instruction_srli_test():
    # TODO
    assert(False)
    return True

def instruction_srai_test():
    # TODO
    assert(False)
    return True

def instruction_slti_test():
    # TODO
    assert(False)
    return True

def instruction_sltiu_test():
    # TODO
    assert(False)
    return True

# load instructions
def instruction_lb(des: int, reg: int, imm: int):
    mem_address = GP_REGS[reg] + imm
    GP_REGS[des] = int.from_bytes(MEM[mem_address: mem_address + 1], "big", signed=True)

def instruction_lh(des: int, reg: int, imm: int):
    mem_address = GP_REGS[reg] + imm
    GP_REGS[des] = int.from_bytes(MEM[mem_address: mem_address + 2], "big", signed=True)

def instruction_lw(des: int, reg: int, imm: int):
    mem_address = GP_REGS[reg] + imm
    GP_REGS[des] = int.from_bytes(MEM[mem_address: mem_address + 4], "big", signed=True)

def instruction_lbu(des: int, reg: int, imm: int):
    mem_address = GP_REGS[reg] + imm
    GP_REGS[des] = int.from_bytes(MEM[mem_address: mem_address + 1], "big", signed=False)

def instruction_lhu(des: int, reg: int, imm: int):
    mem_address = GP_REGS[reg] + imm
    GP_REGS[des] = int.from_bytes(MEM[mem_address: mem_address + 2], "big", signed=False)

# load instruction tests
# TODO

# store instructions

def store_instruction(args: tuple, length: int):
    reg, des, offset = args
    raw_bytes = int(GP_REGS[reg]).to_bytes(length, "big")
    for i in range(length):
        MEM[GP_REGS[des] + offset + i: GP_REGS[des] + offset + i + 1] = raw_bytes[i: i+1]

def instruction_sb(reg: int, des: int, offset: int):
    store_instruction((reg, des, offset), 1)

def instruction_sh(reg: int, des: int, offset: int):
    store_instruction((reg, des, offset), 2)

def instruction_sw(reg: int, des: int, offset: int):
    store_instruction((reg, des, offset), 4)

# store instruction tests
# TODO

# branch instructions

def instruction_beq(reg1: int, reg2: int, target: int):
    global PC
    if GP_REGS[reg1] == GP_REGS[reg2]:
        PC += target

def instruction_bne(reg1: int, reg2: int, target: int):
    global PC
    if GP_REGS[reg1] != GP_REGS[reg2]:
        PC += target

def instruction_blt(reg1: int, reg2: int, target: int):
    global PC
    if GP_REGS[reg1] < GP_REGS[reg2]:
        PC += target

def instruction_bge(reg1: int, reg2: int, target: int):
    global PC
    if GP_REGS[reg1] >= GP_REGS[reg2]:
        PC += target

def instruction_bltu(reg1: int, reg2: int, target: int):
    # TODO differentiate from signed instructions
    global PC
    if GP_REGS[reg1] < GP_REGS[reg2]:
        PC += target

def instruction_bgeu(reg1: int, reg2: int, target: int):
    # TODO differentiate from signed instructions
    global PC
    if GP_REGS[reg1] >= GP_REGS[reg2]:
        PC += target

# branch instruction tests
# TODO

UNIT_TEST_LIST = [
    instruction_add_test,
    instruction_sub_test,
    instruction_xor_test,
    instruction_or_test,
    instruction_and_test,
    instruction_sll_test,
    instruction_srl_test,
    instruction_sra_test,
    instruction_slt_test,
    instruction_sltu_test,
    instruction_addi_test,
    instruction_xori_test,
    instruction_ori_test,
    instruction_andi_test,
    instruction_slli_test,
    instruction_srli_test,
    instruction_srai_test,
    instruction_slti_test,
    instruction_sltiu_test
]

# Tests that utilize external assembly files

def add_test():
    with open('add_test.S', 'r') as test_file:
        instructions = parse_input(test_file.read())
        interpret(instructions, len(instructions))
        assert(GP_REGS[1] == 15)

def sub_test():
    with open('sub_test.S', 'r') as test_file:
        instructions = parse_input(test_file.read())
        interpret(instructions, len(instructions))
        assert(GP_REGS[1] == 5)

def xor_test():
    with open('xor_test.S', 'r') as test_file:
        instructions = parse_input(test_file.read())
        interpret(instructions, len(instructions))
        assert(GP_REGS[1] == 170)

TEST_LIST = [
    add_test,
    sub_test,
    xor_test
]

def run_tests(tests):
    for test in tests:
        try:
            test()
            print("Test {} passed!".format(test.__repr__()))
        except AssertionError:
            print("Test {} failed!".format(test.__repr__()))
        reset_interpreter()

def main():
    pass

if __name__ == '__main__':
    run_tests(UNIT_TEST_LIST)
    run_tests(TEST_LIST)
    main()
