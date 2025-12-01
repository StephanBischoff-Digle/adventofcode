#ifndef STBI_PARSING
#define STBI_PARSING

#include <charconv>
#include <cstdint>
#include <iostream>
#include <optional>
#include <string_view>

namespace stbi::advent {
    std::optional<int32_t> parse_line(std::string_view line) {
        const bool POSITIVE = (line[0] == 'R');

        line.remove_prefix(1);

        int32_t amount{};
        auto [ptr, ec] = std::from_chars(line.data(),
                                         line.data() + line.size(),
                                         amount);
        if (ec == std::errc()) {
            return  POSITIVE ? amount : -amount;
        } else if (ec == std::errc::invalid_argument) {
            std::cout << "This is not a number.\n";
        } else if (ec == std::errc::result_out_of_range) {
            std::cout << "This number is larger than an int.\n";
        }
        return std::nullopt;
    }

    template<typename T>
    T mod(T x, T base) {
        return (x % base + base) % base;
    }
}  // namespace stbi::advent

#endif
