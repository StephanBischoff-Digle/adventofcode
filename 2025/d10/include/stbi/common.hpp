#pragma once

#include <charconv>
#include <optional>
#include <string_view>

namespace stbi {
    template <typename T>
    [[nodiscard]]
    std::optional<T> parse_int(std::string_view s) {
        T x{ 0 };
        // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-pointer-arithmetic)
        auto r = std::from_chars(s.data(), s.data() + s.size(), x);
        if (r.ec != std::errc()) {
            return std::nullopt;
        }
        return x;
    }

    template <typename T>
    [[nodiscard]]
    T udelta(T a, T b) {
        return std::max(a, b) - std::min(a, b);
    }

}  // namespace stbi
