#include <fstream>
#include <iostream>
#include <span>
#include <vector>

bool reachable(const std::vector<std::string>& grid, size_t x, size_t y) {
    int n{ 0 };
    if (x > 0) {
        n += grid[x - 1][y] == '@' ? 1 : 0;

        if (y > 0) {
            n += grid[x - 1][y - 1] == '@' ? 1 : 0;
        }
        if (y < grid[x - 1].length()-1) {
            n += grid[x - 1][y + 1] == '@' ? 1 : 0;
        }
    }
    if (x < grid.size()-1) {
        n += grid[x + 1][y] == '@' ? 1 : 0;

        if (y > 0) {
            n += grid[x + 1][y - 1] == '@' ? 1 : 0;
        }
        if (y < grid[x + 1].length()-1) {
            n += grid[x + 1][y + 1] == '@' ? 1 : 0;
        }
    }

    if (y > 0) {
        n += grid[x][y - 1] == '@' ? 1 : 0;
    }
    if (y < grid[x].length()-1) {
        n += grid[x][y + 1] == '@' ? 1 : 0;
    }

    return n < 4;
}

int main(int argc, char* argv[]) {
    std::span<char*> args(argv, static_cast<size_t>(argc));

    if (argc == 1) {
        std::cout << "Error: input file required!\n";
        return 0;
    }

    std::vector<std::string> grid;

    std::ifstream infile{ args[1] };
    std::string   line;
    while (std::getline(infile, line)) {
        grid.emplace_back(line);
    }

    size_t acc{ 0 };
    for (size_t x{ 0 }; x < grid.size(); x++) {
        for (size_t y{ 0 }; y < grid[x].length(); y++) {
            if (grid[x][y] == '@' && reachable(grid, x, y)) {
                acc += 1;
                std::cout << "\033[33mx\033[0m";
            } else {
                std::cout << grid[x][y];
            }
        }
        std::cout << "\n";
    }

    std::cout << "\nSolution: " << acc << "\n";
}
