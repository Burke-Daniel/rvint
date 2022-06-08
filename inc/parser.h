#ifndef PARSER_H
#define PARSER_H

#include <string>
#include <vector>

typedef std::string Symbol;
typedef Symbol Label;

struct Directive
{
    Symbol directive;
    std::vector<Symbol> args;
};

struct Operand
{
    std::vector<Symbol> symbols;
};

struct Instruction
{
    Symbol instruction;
    Operand operands;
};


class Parser
{
public:
    void Parse(std::vector<std::string> tokens);

private:
    void label();
    void directive();
    void instruction();
    void operand();
};

#endif
