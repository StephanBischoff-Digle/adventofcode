#pragma once

#include <algorithm>
#include <cstdint>
#include <optional>
#include <ostream>

namespace stbf {
using u32 = std::uint32_t;

struct Range {
  u32 start;
  u32 length;

  [[nodiscard]] bool overlaps(const Range &other) const {
    auto a = start <= other.start && other.start <= start + length;
    auto b = start >= other.start && other.start + other.length >= start;
    return a || b;
  }

  [[nodiscard]] std::optional<Range> split_at(u32 index) {
    if (index < start || index >= start + length) {
      return std::nullopt;
    }

    auto new_length = index - start;
    auto rem_length = length - new_length - 1;
    auto new_start = start + new_length + 1;

    length = new_length;
    return {{.start = new_start, .length = rem_length}};
  }

  [[nodiscard]] static std::optional<Range> merge(const Range &a,
                                                  const Range &b) {
    if (!a.overlaps(b)) {
      return std::nullopt;
    }

    auto start = std::min(a.start, b.start);
    auto end = std::max(a.start + a.length, b.start + b.length);
    return {{.start = start, .length = end - start}};
  }
};

bool operator<(const Range &a, const Range &b) { return a.start < b.start; }
} // namespace stbf
//

std::ostream &operator<<(std::ostream &os, const stbf::Range &range) {
  os << range.start << " - " << range.start + range.length << " ("
     << range.length + 1 << ")";
  return os;
}
