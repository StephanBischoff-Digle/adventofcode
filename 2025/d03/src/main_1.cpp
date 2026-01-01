#include <iostream>
#include <span>
#include <fstream>

int main(int argc, char* argv[]) {

    std::span<char *> args(argv, static_cast<size_t>(argc));

    if (argc == 1) {
        std::cout << "Error: input file required!\n";
        return 0;
    }

    std::ifstream infile{ args[1] };
    std::string line;
    int acc {0};

    while (std::getline(infile, line)) {
        std::cout << line << "\n";

        char max_l {0};
        size_t idx_l {0};
        char max_r {0};

        for (size_t i {0}; i < line.length() - 1; i++) {
            if (line[i] > max_l) {
                idx_l = i;
                max_l = line[i];
            }
        }

        for (size_t i {idx_l + 1}; i < line.length(); i++) {
            max_r = std::max(max_r, line[i]);
        }

        acc += (max_l - '0') * 10 + (max_r - '0');
    }

    std::cout << acc << "\n";
}
