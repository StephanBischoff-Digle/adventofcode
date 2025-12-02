#pragma once

#include <fstream>
#include <optional>

#include "common.hpp"
#include "interval.hpp"
#include "parsing.hpp"

namespace advent::naive {
    /**
     * Iterates over the interval and accumulates the numbers that match the problem pattern.
     * @param ival The interval to be checked.
     * @returns The accumulated pattern numbers.
     */
    [[nodiscard]] static constexpr std::size_t accumulate_interval(const advent::Interval& ival) noexcept {
        std::size_t acc{ 0 };
        for (std::size_t i{ ival.d_start }; i <= ival.d_end; i++) {
            if (advent::is_pattern(i)) {
                acc += i;
            }
        }
        return acc;
    }

    /**
     * The navie solution for part 1.
     *
     * This solution basically just iterates over each range (interval) and check each number in the range,
     * if it's a pattern. In case, it's a pattern, the number is added to the accumulator.
     * The function may return nullopt in case the input is invalid.
     *
     * @param infile The input file.
     * @returns The solution for Day 2 Part 1 or nullopt if the input was invalid.
     */
    [[nodiscard]] std::optional<std::size_t> solve(std::ifstream& infile) noexcept {
        std::size_t acc{ 0 };

        std::string line;
        while (std::getline(infile, line, ',')) {
            std::optional<advent::Interval> interval = advent::parse_line(std::string_view{ line });
            if (!interval) {
                return std::nullopt;
            }
            acc += accumulate_interval(*interval);
        }

        return acc;
    }

}  // namespace advent::naive
