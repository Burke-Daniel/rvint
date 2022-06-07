#include <iostream>
#include <fstream>
#include <sstream>
#include <string>

enum class TokenType
{
    Newline,
    Comment,
    Symbol
};

struct Token
{
    TokenType type;
    std::string token;
};

void parse_input(std::ifstream& file)
{
    std::string line;

    while (std::getline(file, line)) 
    {
        std::cout << line << std::endl;
    }
}

int main(int argc, char* argv[])
{
    if (argc < 2) { return 1; }

    std::ifstream file(argv[1]);

    if (!file.is_open()) { return 1; }

    parse_input(file);

    return 0;
}
