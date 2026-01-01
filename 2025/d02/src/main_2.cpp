#include <algorithm>
#include <fstream>
#include <iostream>
#include <optional>
#include <ranges>
#include <span>
#include <string>

#include "interval.hpp"
#include "parsing.hpp"

/**
 * Checks whether the given number is a pattern as of the assignment.
 */
[[nodiscard]] bool is_pattern(std::size_t x) {
    // Numbers below 10 cannot repeat their digits twice.
    if (x < 10) {
        return false;
    }

    // Convert to string
    auto        str_x = std::to_string(x);
    std::size_t half{ str_x.size() / 2 };

    // Cut the string into non-overlapping chunks of ever increasing width.
    for (auto const width : std::views::iota(1U, half + 1)) {
        // Reject sizes, that don't fill the string completely
        if (str_x.size() % width != 0) {
            continue;
        }
        auto const chunks        = str_x | std::views::chunk(width);
        auto const initial_chunk = chunks.front();

        // check if all chunks are equal to the first one
        bool pattern = std::ranges::all_of(chunks, [&initial_chunk](const auto candidate) {
                           return std::ranges::equal(candidate, initial_chunk);
                       });

        // If we found a pattern, we can report
        if (pattern) {
            return true;
        }
    }

    return false;
}

int main(int argc, char* argv[]) {
    std::span<char*> args(argv, static_cast<size_t>(argc));

    if (argc == 1) {
        std::cout << "Error: input file required!\n";
        return 0;
    }

    std::ifstream infile{ args[1] };
    std::string   line;
    std::size_t   acc{ 0 };

    while (std::getline(infile, line, ',')) {
        // parsing the line into two numbers
        std::optional<advent::Interval> interval = advent::parse_line(std::string_view{ line });
        if (!interval) {
            std::cerr << "Failed to parse an interval\n";
            return -1;
        }

        // iterate over the whole interval and check each number
        for (std::size_t v : std::views::iota(interval->d_start, interval->d_end + 1)) {
            if (is_pattern(v)) {
                acc += v;
            }
        }
    }

    std::cout << "Solution: " << acc << "\n";
    return 0;
}
