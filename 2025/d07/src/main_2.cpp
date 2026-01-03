#include <algorithm>
#include <fstream>
#include <iostream>
#include <span>
#include <string>
#include <vector>

int main(int argc, char* argv[]) {
    std::span<char*> args(argv, static_cast<size_t>(argc));

    if (args.size() == 1) {
        std::cout << "Error: input file required!\n";
        return -1;
    }

    std::ifstream infile{ args[1] };
    std::string   line;
    std::string   prev_line;

    // keep track of the timeliens that reached this column
    std::vector<uint64_t> local_timelines{};

    // get first line
    std::getline(infile, prev_line);
    local_timelines.reserve(prev_line.length());
    for (size_t i{ 0 }; i < prev_line.length(); i++) {
        // We start with a single timeline at the start
        local_timelines.push_back(prev_line[i] != 'S' ? 0 : 1);
    }

    while (std::getline(infile, line)) {
        for (size_t i{ 0 }; i < line.length(); i++) {
            if (line[i] == '.') {
                if (prev_line[i] == '|' || prev_line[i] == 'S') {
                    line[i] = '|';
                }
            }

            // If a splitter is hit, all the timelines that hit the splitter
            // can go to the left and to the right.
            if (line[i] == '^' && prev_line[i] == '|') {
                if (i > 0) {
                    line[i - 1] = '|';
                    local_timelines[i - 1] += local_timelines[i];
                }
                if (i < line.length()) {
                    line[i + 1] = '|';
                    local_timelines[i + 1] += local_timelines[i];
                }
                local_timelines[i] = 0;
            }
        }
        prev_line = line;
    }

    // count all the timelines
    std::cout << "Solution: " << std::ranges::fold_left(local_timelines, 0, std::plus<>()) << "\n";
}
