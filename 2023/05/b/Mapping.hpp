#pragma once

#include "Range.hpp"
#include <cstdint>
#include <string>

namespace stbf {

using u32 = std::uint32_t;
using i64 = std::int64_t;

struct Mapped {
  Range mapped;
  std::optional<Range> prior = std::nullopt;
  std::optional<Range> posterior = std::nullopt;
};

class Mapping {
  Range from;
  Range to;
  i64 delta;

public:
  Mapping(u32 from, u32 to, u32 length)
      : from{from, length}, to{to, length},
        delta{static_cast<i64>(to) - static_cast<i64>(from)} {}

  [[nodiscard]] bool can_map(const Range &range) const {
    return range.overlaps(from);
  }

  [[nodiscard]] std::optional<Mapped> map(Range range) const {
    if (!can_map(range)) {
      return std::nullopt;
    }

    /*
     * |       | from      |            |
     *
     * | range                          |
     * | range | a_split                |
     * | range | a_split   | post       |
     */
    auto a_split = range.split_at(from.start);
    if (a_split) {
      auto post = a_split->split_at(from.start + from.length);

      a_split->start = to.start;
      return {{
          .mapped = *a_split,
          .prior = range,
          .posterior = post,
      }};
    }

    auto mapped_start = static_cast<u32>(range.start + delta);

    /*
     * |       | from      |            |
     *
     * |       | range                  |
     * |       | range     | b_split    |
     */
    auto b_split = range.split_at(from.start + from.length);
    range.start = mapped_start;

    return {{
        .mapped = range,
        .prior = std::nullopt,
        .posterior = b_split,
    }};
  }

  [[nodiscard]] static Mapping parse(std::string line) {
    size_t pos = line.find(' ');
    u32 d = std::stol(line.substr(0, pos));
    std::stol(line.substr(0, pos));
    line.erase(0, pos + 1);

    pos = line.find(' ');
    u32 s = std::stol(line.substr(0, pos));
    std::stol(line.substr(0, pos));
    line.erase(0, pos + 1);

    u32 r = std::stol(line.substr(0, std::string::npos));
    std::stol(line.substr(0, pos));

    return Mapping(s, d, r - 1);
  }

  bool operator<(const Mapping &other) const { return from < other.from; }

  friend std::ostream &operator<<(std::ostream &os,
                                  const stbf::Mapping &mapping) {
    os << mapping.from << " -> " << mapping.to;
    return os;
  }
};

} // namespace stbf
