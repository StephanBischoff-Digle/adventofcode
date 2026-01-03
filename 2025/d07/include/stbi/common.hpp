#pragma once

#include <charconv>
#include <optional>
#include <string_view>

namespace stbi {
    template <typename T>
    std::optional<T> parse_int(std::string_view s) {
        T x{ 0 };
        // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-pointer-arithmetic)
        auto r = std::from_chars(s.data(), s.data() + s.size(), x);
        if (r.ec != std::errc()) {
            return std::nullopt;
        }
        return x;
    }
}  // namespace stbi
