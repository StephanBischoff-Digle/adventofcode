#ifndef STBI_PARSING
#define STBI_PARSING

#include <charconv>
#include <cstdint>
#include <optional>
#include <string_view>

namespace stbi::advent {
    std::optional<int32_t> parse_line(std::string_view line) {
        const bool POSITIVE = (line[0] == 'R');

        line.remove_prefix(1);

        int32_t amount{};
        auto [_, ec] = std::from_chars(line.data(),
                                       line.data() + line.size(),
                                       amount);
        if (ec == std::errc()) {
            return POSITIVE ? amount : -amount;
        }
        return std::nullopt;
    }

    template <typename T>
    T mod(T x, T base) {
        return (x % base + base) % base;
    }
}  // namespace stbi::advent

#endif
