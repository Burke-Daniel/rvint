#include <algorithm>
#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>

#include <stdio.h>
#include <string.h>


// Lexer reads token char-by-char to see what type of token it will be
// Basically should remove comments from being parsed

std::vector<std::string> tokens;

void print_tokens()
{
    std::cout << "Tokens: { ";
    for (const auto& token : tokens) { std::cout << token << " "; }
    std::cout << "}\n";
}

bool has_char(const std::string& s, char c)
{
    return s.find(c) != std::string::npos;
}

void remove_from_string(std::string& s, char c)
{
    s.erase(std::remove(s.begin(), s.end(), c), s.end());
}

void tokenize(char* line)
{
    char* curr_token;
    curr_token = strtok(line, " ,:()");
    while (curr_token != nullptr)
    {
        tokens.push_back(curr_token);
        curr_token = strtok(nullptr, " ,:()");
    }
}

void parse_input(std::ifstream& file)
{
    std::string line;

    while (std::getline(file, line)) 
    {
        char tokenizable_line[512];
        strcpy(tokenizable_line, line.c_str());
        tokenize(tokenizable_line);
    }
    print_tokens();
}

int main(int argc, char* argv[])
{
    if (argc < 2) { return 1; }

    std::ifstream file(argv[1]);

    if (!file.is_open()) { return 1; }

    parse_input(file);

    return 0;
}
