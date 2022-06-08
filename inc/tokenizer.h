#ifndef TOKENIZER_H
#define TOKENIZER_H

#include <string>
#include <vector>

class Tokenizer
{
public:
    std::vector<std::vector<std::string>> tokens;

    void print_tokens();
    void tokenize(char* line);
};

#endif
