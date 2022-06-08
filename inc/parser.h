#ifndef PARSER_H
#define PARSER_H

#include <memory>
#include <string>
#include <vector>

typedef std::string Symbol;

// How do we store tokens in memory?
//
// - Need to know what token is next
// - Performance? Probably don't think about that for now
// - Trivial way, make all these inherit from a common
//   base where the base just contains a data
//   member for the type

enum class TokenType
{
    Label,
    Directive,
    Instruction,
    Operand,
};

struct Token
{
public:
    TokenType type;
};

struct Label : public Token
{
    Symbol label;
};

struct Directive : public Token
{
    Symbol directive;
    std::vector<Symbol> args;
};

struct Instruction : public Token
{
    Symbol instruction;
    std::vector<Symbol> operands;
};


class Parser
{
public:
    std::vector<std::unique_ptr<Token>> tokens;    

    void Parse(const std::vector<std::string>& t);

private:
    void add_label(const Symbol&);

    void label();
    void directive();
    void instruction();
    void operand();
};

#endif
