#include <fstream>
#include <iostream>
#include <string>
#include <string_view>

#include "parsing.hpp"


int main(int argc, char* argv[]) {
    if (argc == 1) {
        std::cout << "Error: input file required!\n";
        return 0;
    }

    std::ifstream infile{ argv[1] };
    int32_t           dial    = 50;
    uint32_t           counter = 0;

    std::string line;
    while (std::getline(infile, line)) {
        auto parsed = stbi::advent::parse_line(std::string_view{ line });
        if (!parsed) {
            std::cout << "Abort processing\n";
            return 1;
        }

        dial = stbi::advent::mod(dial + *parsed, 100);
        if (dial == 0) {
            counter++;
        }
    }
    std::cout << counter << '\n';
}
