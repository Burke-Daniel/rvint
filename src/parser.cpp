#include "parser.h"

#include <algorithm>
#include <iostream>

static bool is_label(const Symbol& symbol)
{
    if (symbol.find(':') != std::string::npos)
    {
        return true;
    }
    return false;
}

static bool is_directive(const Symbol& symbol)
{
    if (symbol[0] == '.')
    {
        return true;
    }
    return false;
}

static bool is_comment(const Symbol& symbol)
{
    if (symbol[0] == '#')
    {
        return true;
    }
    return false;
}

void Parser::add_label(const Symbol& symbol)
{

    auto label = std::make_unique<Label>();

    label->type = TokenType::Label;
    label->label = symbol;
    label->label.erase(std::remove(label->label.begin(), label->label.end(), ':'), label->label.end());

    std::cout << "Added Label with value: " << label->label << '\n';

    tokens.push_back(std::move(label));
}

void Parser::Parse(const std::vector<std::string>& t)
{
    for (int i = 0; i < t.size();)
    {
        if (is_comment(t[i]))
        {
            break;
        }
        else if (is_label(t[i]))
        {
            add_label(t[i]);
            i++;
            // TODO label may have more stuff after it
        }
        else if (is_directive(t[i]))
        {
            auto directive = std::make_unique<Directive>();

            directive->type = TokenType::Directive;
            directive->directive = t[i++];

            std::cout << "Added directive with directive: " << directive->directive << " and args:\n";

            for (; i < t.size() && !is_comment(t[i]); i++)
            {
                directive->args.push_back(t[i]);    
                std::cout << t[i] << "\n";
            }

            std::cout << '\n';

            tokens.push_back(std::move(directive));

            if (i < t.size() && is_comment(t[i]))
            {
                break;
            }

            i++;
        }
        else // is_instruction
        {
            auto instruction = std::make_unique<Instruction>();

            instruction->type = TokenType::Instruction;
            instruction->instruction = t[i++];

            std::cout << "Added instruction with instruction name: " << instruction->instruction << " and operands:\n";
            for (; i < t.size() && !is_comment(t[i]); i++)
            {
                instruction->operands.push_back(t[i]);    
                std::cout << t[i] << "\n";
            }

            std::cout << '\n';

            tokens.push_back(std::move(instruction));

            if (i < t.size() && is_comment(t[i]))
            {
                break;
            }

            i++;
        }
    }
}

