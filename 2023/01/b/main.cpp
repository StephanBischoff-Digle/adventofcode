#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <numeric>
#include <set>
#include <string>

using u32 = uint32_t;

/// Store the position of the value string and the value
struct PosVal {
  size_t pos;
  u32 val;
};

/// Comparator for the PostVal-set, which determines order by position
struct PosValComparator {
  bool operator()(const PosVal &a, const PosVal &b) const {
    return a.pos < b.pos;
  }
};

int main(int argc, char *argv[]) {
  // Read file
  if (argc == 1) {
    std::cout << "Requires input file" << std::endl;
    return 0;
  }

  /// Lookup table for the strings and their corresponding values
  const std::pair<const char *, u32> words[] = {
      {"1", 1},     {"2", 2},     {"3", 3},    {"4", 4},    {"5", 5},
      {"6", 6},     {"7", 7},     {"8", 8},    {"9", 9},    {"one", 1},
      {"two", 2},   {"three", 3}, {"four", 4}, {"five", 5}, {"six", 6},
      {"seven", 7}, {"eight", 8}, {"nine", 9},
  };

  std::ifstream infile(argv[1]);

  u32 accumulator = 0;
  std::string line;

  while (std::getline(infile, line)) {
    std::set<PosVal, PosValComparator> pv;
    // std::cout << "\n\n" << line << "\n";

    for (const auto &kv : words) {
      // find first occurence
      auto pos = line.find(kv.first, 0);
      if (pos != std::string::npos) {
        pv.insert({pos, kv.second});
      }

      // find last occurence
      pos = line.rfind(kv.first);
      if (pos != std::string::npos) {
        pv.insert({pos, kv.second});
      }
    }

    // for (const auto &posval : pv) {
    //   std::cout << posval.val;
    // }

    u32 first = pv.begin()->val;
    u32 last = std::prev(pv.end())->val;

    accumulator += 10 * first + last;
  }

  std::cout << "\n" << accumulator << std::endl;

  return 0;
}
