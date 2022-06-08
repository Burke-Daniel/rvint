#include <algorithm>
#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>

#include <stdio.h>
#include <string.h>

#include "parser.h"
#include "tokenizer.h"

class Rvint
{
public:
    Tokenizer tokenizer;
    Parser parser;

    void parse_input(std::ifstream& file)
    {
        std::string line;

        while (std::getline(file, line)) 
        {
            // TODO check that this buffer is not overflown
            char tokenizable_line[512];
            strcpy(tokenizable_line, line.c_str());
            tokenizer.tokenize(tokenizable_line);
        }

        for (const auto& line : tokenizer.tokens)
        {
            parser.Parse(line);
        }
    }
};


int main(int argc, char* argv[])
{
    if (argc < 2) { return 1; }

    std::ifstream file(argv[1]);

    if (!file.is_open()) { return 1; }

    Rvint app;

    app.parse_input(file);

    return 0;
}
