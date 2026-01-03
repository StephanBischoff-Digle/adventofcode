#include <fstream>
#include <iostream>
#include <span>
#include <string>

int main(int argc, char* argv[]) {
    std::span<char*> args(argv, static_cast<size_t>(argc));

    if (args.size() == 1) {
        std::cout << "Error: input file required!\n";
        return -1;
    }

    std::ifstream infile{ args[1] };
    std::string   line;
    std::string   prev_line;

    uint64_t splits{ 0 };

    // get first line
    std::getline(infile, prev_line);
    while (std::getline(infile, line)) {
        for (size_t i{ 0 }; i < line.length(); i++) {
            if (line[i] == '.') {
                if (prev_line[i] == '|' || prev_line[i] == 'S') {
                    line[i] = '|';
                }
            }
            if (line[i] == '^' && prev_line[i] == '|') {
                splits += 1;

                if (i > 0 && line[i - 1] == '.') {
                    line[i - 1] = '|';
                }
                if (i < line.length() && line[i + 1] == '.') {
                    if (prev_line[i + 1] != '|') {
                        line[i + 1] = '|';
                    }
                }
            }
        }
        prev_line = line;
    }

    std::cout << "Solution: " << splits << "\n";
}
