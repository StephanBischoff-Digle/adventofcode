#include <fstream>
#include <iostream>

#include "naive.hpp"

int main(int argc, char* argv[]) {
    if (argc == 1) {
        std::cout << "Error: input file required!\n";
        return 0;
    }

    std::ifstream infile{ argv[1] };

    auto result = advent::naive::solve(infile);
    if (!result) {
        std::cerr << "Failed to compute result\n";
        return 1;
    }

    std::cout << result.value() << '\n';
}
