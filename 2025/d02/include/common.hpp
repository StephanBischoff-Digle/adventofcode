#pragma once

#include <cstddef>
#include <string>

namespace advent {
    /**
     * Checks, if the number is a pattern as outlined by Part 1.
     * @param x The number to be evaluated.
     * @returns true if x is a pattern.
     */
    [[nodiscard]] bool is_pattern(std::size_t x) {
        auto str_x = std::to_string(x);
        if (str_x.size() % 2 == 1) {
            return false;
        }
        std::size_t half{ str_x.size() / 2 };

        for (std::size_t i{ 0 }; i < half; i++) {
            if (str_x[i] != str_x[half + i]) {
                return false;
            }
        }
        return true;
    }
}  // namespace advent
