#include "tokenizer.h"

#include <cstdio>
#include <cstring>
#include <iostream>

void Tokenizer::print_tokens()
{
    for (int i = 0; i < tokens.size(); i++)
    {
        std::cout << "Line " << i << ":\n";
        for (const auto& token : tokens[i])
        {
            std::cout << token << '\n';
        }
        std::cout << '\n';
    }
}

void Tokenizer::tokenize(char* line)
{
    char* curr_token;
    curr_token = strtok(line, " ,:()");
    std::vector<std::string> line_of_tokens;
    while (curr_token != nullptr)
    {
        line_of_tokens.push_back(curr_token);
        curr_token = strtok(nullptr, " ,:()");
    }
    tokens.push_back(line_of_tokens);
}

