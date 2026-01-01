#include <array>
#include <fstream>
#include <iostream>
#include <span>

static const std::array<uint64_t, 12> EXP{
    1,         10,         100,         1'000,         10'000,         100'000,
    1'000'000, 10'000'000, 100'000'000, 1'000'000'000, 10'000'000'000, 100'000'000'000
};

int main(int argc, char* argv[]) {
    std::span<char*> args(argv, static_cast<size_t>(argc));

    if (argc == 1) {
        std::cout << "Error: input file required!\n";
        return 0;
    }

    std::ifstream infile{ args[1] };
    std::string   line;
    uint64_t      acc{ 0 };

    while (std::getline(infile, line)) {
        std::cout << "\n" << line << "\n";
        uint64_t joltage{ 0 };

        size_t start_idx{ 0 };
        for (size_t remaining{ 12 }; remaining > 0; remaining--) {
            char current_max{ 0 };
            for (size_t i{ 0 }; i < start_idx; i++) {
                std::cout << " ";
            }
            for (size_t i{ start_idx }; i < line.length() - remaining + 1; i++) {
                if (current_max < line[i]) {
                    current_max = line[i];
                    start_idx   = i + 1;
                    std::cout << "\033[34m" << line[i] << "\033[0m";
                } else {
                    std::cout << "\033[38;5;240m" << line[i] << "\033[0m";
                }
            }

            for (size_t i{ 0 }; i < remaining - 1; i++) {
                std::cout << "\033[38;5;9mx\033[0m";
            }
            std::cout << "\n";

            joltage += static_cast<uint64_t>(current_max - '0') * EXP.at(remaining - 1);
        }
        std::cout << "\033[33m" << joltage << "\033[0m\n";
        acc += joltage;
    }

    std::cout << "\nSolution: " << acc << "\n";
}
