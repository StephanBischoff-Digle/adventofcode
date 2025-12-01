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
    int32_t       dial    = 50;
    uint32_t      counter = 0;

    std::string line;
    while (std::getline(infile, line)) {
        auto parsed = stbi::advent::parse_line(std::string_view{ line });
        if (!parsed) {
            std::cout << "Abort processing\n";
            return 1;
        }
        
        // If the dial lands on 0 and the next input is a negative number, we don't want to count passing 0.
        if (dial == 0 && *parsed < 0) {
            dial += 100;
        }

        // The input may be more than one dial revolution.
        if (std::abs(*parsed) >= 100) {
            counter += static_cast<uint32_t>(std::abs(*parsed) / 100);
            *parsed = *parsed % 100;
        }

        // Check, if the dial passes the 0 by checking, if the modulus is different from the simple addition.
        dial += *parsed;
        auto tmp_dial = stbi::advent::mod(dial, 100);
        if (tmp_dial != dial || tmp_dial == 0) {
            counter++;
            dial = tmp_dial;
        }
    }

    std::cout << counter << '\n';
}
