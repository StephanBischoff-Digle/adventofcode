#include <fstream>
#include <iostream>

#include "naive.hpp"

int main(int argc, char* argv[]) {
    std::span<char *> args(argv, static_cast<size_t>(argc));

    if (argc == 1) {
        std::cout << "Error: input file required!\n";
        return 0;
    }

    std::ifstream infile{ args[1] };

    auto result = advent::naive::solve(infile);
    if (!result) {
        std::cerr << "Failed to compute result\n";
        return 1;
    }

    std::cout << result.value() << '\n';
}
