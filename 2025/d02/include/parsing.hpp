#pragma once

#include <charconv>
#include <optional>
#include <ranges>
#include <string_view>

#include "interval.hpp"

namespace advent {
    std::optional<advent::Interval> parse_line(std::string_view line) {
        auto parts = std::views::split(line, '-') |
                     std::views::transform([](auto&& part) {
                         return std::string_view(
                             &*part.begin(),
                             static_cast<size_t>(std::ranges::distance(part)));
                     });

        std::size_t start{};
        std::size_t end{};
        auto        first = parts.front();
        auto        last  = *std::ranges::next(parts.begin(), 1);

        auto r = std::from_chars(first.data(),
                                 first.data() + first.size(),
                                 start);
        if (r.ec != std::errc()) {
            return std::nullopt;
        }
        r = std::from_chars(last.data(), last.data() + last.size(), end);
        if (r.ec != std::errc()) {
            return std::nullopt;
        }

        return advent::Interval{ start, end };
    }
}  // namespace advent
