#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <span>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using u32 = std::uint32_t;

struct Point {
  size_t x;
  size_t y;

  /**
   * Checks wether two Points are next to each other.
   * @param o The other point.
   * @return True if the points are close to each other.
   */
  const bool in_range(const Point &o) const {
    auto max_x = std::max(o.x, x);
    auto min_x = std::min(o.x, x);

    auto max_y = std::max(o.y, y);
    auto min_y = std::min(o.y, y);

    return (max_x - min_x) <= 1 && (max_y - min_y) <= 1;
  }

  const bool operator==(const Point &o) const { return o.x == x && o.y == y; }
};

/**
 * Calculates a hash for the Point. The assumption is, that the coordinates
 * will not exceed 16 bit values per coordniate.
 * @return A hash for the Point.
 */
const auto point_hash = [](const Point &p) { return p.x << (size_t)16 | p.y; };

struct NumberPoint {
  u32 v;
  size_t digits;
  Point pos;

  std::vector<Point> digit_points() const {
    std::vector<Point> points;
    points.reserve(digits);
    for (auto i = 0; i < digits; i++) {
      points.push_back({.x = pos.x + i, .y = pos.y});
    }
    return points;
  }
};

int main(int argc, char *argv[]) {
  auto args = std::span(argv, size_t(argc));

  // Read file
  if (argc == 1) {
    std::cout << "Requires input file\n";
    return 0;
  }

  std::ifstream infile(args[1]);

  std::unordered_map<Point, char, decltype(point_hash)> symbols;
  std::vector<NumberPoint> numbers;

  std::string line;
  size_t y = 0;
  while (std::getline(infile, line)) {
    for (size_t x = 0; x < line.length(); x++) {
      auto c = line[x];
      if (c != '*' && !isdigit(c)) {
        continue;
      }

      if (isdigit(c)) {
        auto start = x;
        while (isdigit(c)) {
          x++;
          c = line[x];
        }
        auto delta = x - start;
        const u32 number = std::stol(line.substr(start, delta));
        numbers.push_back(NumberPoint{
            .v = number, .digits = delta, .pos = Point{.x = start, .y = y}});

        // decrement, cause for loop increments on continue!
        x--;
        continue;
      }
      symbols.insert({Point{.x = x, .y = y}, c});
    }
    y++;
  }

  std::unordered_set<size_t> included_indices;
  u32 answer = 0;
  for (const auto &kv : symbols) {
    u32 ratio = 1;
    bool is_gear = false;
    for (auto idx = 0; idx < numbers.size(); idx++) {
      if (included_indices.contains(idx)) {
        continue;
      }

      for (const auto &dp : numbers[idx].digit_points()) {
        if (dp.in_range(kv.first)) {
          if (ratio != 1) {
            is_gear = true;
          }
          included_indices.insert(idx);
          ratio *= numbers[idx].v;
          break;
        }
      }
    }

    if (is_gear) {
      answer += ratio;
    }
  }

  std::cout << answer << '\n';

  return 0;
}
