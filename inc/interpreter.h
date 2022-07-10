#include "parser.h"
#include <cstdint>
#include <cstdlib>
#include <vector>

class Interpreter
{
public:
    void interpret(const Instruction& instruction);

private:

    void load_upper_immediate(int reg, int imm);

    static const int memory_size = 4096;
    size_t program_counter = 0;
    uint32_t gen_regs[32] = {0};
    float fp_regs[32] = {0};
    std::vector<uint8_t> memory;
};