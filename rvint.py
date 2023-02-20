import sys
import os

registers = [0] * 32
pc = 0

def print_registers():
    for idx, reg in enumerate(registers):
        print(f"R{idx}: {reg}")

immediate_operations = [
    "ADDI",
    "SLTI",
    "ANDI",
    "ORI",
    "XORI",
    "SLLI",
    "SRLI",
    "SRAI",
    "LUI",
    "AUIPC",
]

integer_register_operations = [
    "ADD",
    "SLT",
    "SLTU",
    "AND",
    "OR",
    "XOR",
    "SLL",
    "SRL",
    "SUB",
    "SRA",
    "NOP",
]

control_flow_operations = [
    "JAL",
    "JALR",
    "BEQ",
    "BNE",
    "BLT",
    "BGE",
]

load_store_operations = [
    "LOAD",
    "STORE",
]


separator = " "
comment = "#"

if os.name == "nt":
    newline = "\r\n"
else:
    newline = "\n"

program = []

def immediate_instruction(op, dest, r1, imm):
    if op == "ADDI":
        registers[dest] = registers[1] + imm
    if op == "SLTI":
        pass
    if op == "ANDI":
        registers[dest] = registers[1] & imm
    if op == "ORI":
        registers[dest] = registers[1] | imm
    if op == "XORI":
        registers[dest] = registers[1] ^ imm
    if op == "SLLI":
        pass
    if op == "SRLI":
        pass
    if op == "SRAI":
        pass
    if op == "LUI":
        pass
    if op == "AUIPC":
        pass

def integer_register_operation(op, dest, r1, r2):
    if op == "ADD":
        print("Add operation!")
        registers[dest] = registers[r1] + registers[r2]
    if op == "SLT":
        pass
    if op == "SLTU":
        pass
    if op == "AND":
        pass
    if op == "OR":
        pass
    if op == "XOR":
        pass
    if op == "SLL":
        pass
    if op == "SRL":
        pass
    if op == "SUB":
        pass
    if op == "SRA":
        pass
    if op == "NOP":
        pass

def handle_instruction(instruction):
    global pc
    opcode = instruction[0].upper()

    if opcode in immediate_operations:
        dest = int(instruction[1][1])
        r1 = int(instruction[2][1])
        imm = int(instruction[3])
        print(f"dest: {dest}, r1: {r1}, imm: {imm}")
        immediate_instruction(opcode, dest, r1, imm)
        pc += 1
    elif opcode.upper() in integer_register_operations:
        dest = int(instruction[1][1])
        r1 = int(instruction[2][1])
        r2 = int(instruction[3][1])
        print(f"dest: {dest}, r1: {r1}, r2: {r2}")
        integer_register_operation(opcode, dest, r1, r2)
        pc += 1
    elif opcode.upper() in control_flow_operations:
        pass
    elif opcode.upper() in load_store_operations:
        pass
    else:
        print("Invalid instruction!")
        exit()


def main():
    global pc
    argc = len(sys.argv)
    if (argc > 1):
        file_name = sys.argv[1]
        file = open(file_name, "r")
        file_contents = file.read()
        lines = file_contents.splitlines()
        program = [line.split() for line in lines]
        while pc < len(program):
            instruction = program[pc]
            handle_instruction(instruction)
            registers[0] = 0
        print("Program complete!")
        print_registers()
    else:
        print("Error: please provide a file name")



if __name__ == '__main__':
    main()
