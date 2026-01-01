#include <fstream>
#include <iostream>
#include <span>
#include <vector>

#include "gif.h"

void dispatch_frame(GifWriter& g, const std::vector<std::string>& grid) {
    uint32_t             delay  = 10;
    size_t               width  = grid.size();
    size_t               height = grid[0].length();
    std::vector<uint8_t> frame(width * height * 4, 0);

    for (size_t x{ 0 }; x < grid.size(); x++) {
        for (size_t y{ 0 }; y < grid[x].length(); y++) {
            if (grid[x][y] == '@') {
                size_t idx     = 4 * (x * width + y);
                frame[idx + 0] = 20;
                frame[idx + 1] = 76;
                frame[idx + 2] = 100;
            } else if (grid[x][y] == 'x') {
                size_t idx     = 4 * (x * width + y);
                frame[idx + 0] = 249;
                frame[idx + 1] = 133;
                frame[idx + 2] = 45;
            }
        }
    }

    GifWriteFrame(&g, frame.data(), static_cast<uint32_t>(width), static_cast<uint32_t>(height), delay);
}

constexpr int is_roll_or_marked(char c) {
    if (c == '@' || c == 'x') {
        return 1;
    }
    return 0;
}

bool reachable(const std::vector<std::string>& grid, size_t x, size_t y) {
    int n{ 0 };
    if (x > 0) {
        n += is_roll_or_marked(grid[x - 1][y]);

        if (y > 0) {
            n += is_roll_or_marked(grid[x - 1][y - 1]);
        }
        if (y < grid[x - 1].length() - 1) {
            n += is_roll_or_marked(grid[x - 1][y + 1]);
        }
    }
    if (x < grid.size() - 1) {
        n += is_roll_or_marked(grid[x + 1][y]);

        if (y > 0) {
            n += is_roll_or_marked(grid[x + 1][y - 1]);
        }
        if (y < grid[x + 1].length() - 1) {
            n += is_roll_or_marked(grid[x + 1][y + 1]);
        }
    }

    if (y > 0) {
        n += is_roll_or_marked(grid[x][y - 1]);
    }
    if (y < grid[x].length() - 1) {
        n += is_roll_or_marked(grid[x][y + 1]);
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

    auto      gif_file = "animated.gif";
    GifWriter g;
    GifBegin(&g, gif_file, static_cast<uint32_t>(grid.size()), static_cast<uint32_t>(grid[0].length()), 10);

    size_t acc{ 0 };
    bool   reachables_found{ true };
    while (reachables_found) {
        reachables_found = false;
        // Mark and count
        for (size_t x{ 0 }; x < grid.size(); x++) {
            for (size_t y{ 0 }; y < grid[x].length(); y++) {
                if (grid[x][y] == '@' && reachable(grid, x, y)) {
                    acc += 1;
                    grid[x][y]       = 'x';
                    reachables_found = true;
                }
            }
        }
        
        dispatch_frame(g, grid);

        // Cleanup
        for (size_t x{ 0 }; x < grid.size(); x++) {
            for (size_t y{ 0 }; y < grid[x].length(); y++) {
                if (grid[x][y] == 'x') {
                    grid[x][y] = '.';
                }
            }
        }
    }

    GifEnd(&g);

    std::cout << "\nSolution: " << acc << "\n";
}
