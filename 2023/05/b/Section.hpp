#pragma once

#include <algorithm>
#include <deque>
#include <fstream>
#include <queue>
#include <vector>

#include "Mapping.hpp"

namespace stbf {

std::vector<Range> merge_all_ranges(std::vector<Range> ranges) {
  std::priority_queue<Range> to_be_merged(ranges.begin(), ranges.end());
  std::vector<Range> merged;

  Range merger = to_be_merged.top();
  to_be_merged.pop();
  while (!to_be_merged.empty()) {
    auto current = to_be_merged.top();
    to_be_merged.pop();
    auto merged_range = Range::merge(merger, current);
    if (merged_range) {
      merger = *merged_range;
    } else {
      merged.push_back(merger);
      merger = current;
    }

    if (to_be_merged.empty()) {
      merged.push_back(merger);
    }
  }

  return merged;
}

struct Section {
  std::vector<Mapping> mappings;

  [[nodiscard]] std::vector<Range> map(const std::vector<Range> &ranges) const {
    // assume sorted ranges
    std::deque<Range> unmapped(ranges.begin(), ranges.end());
    std::vector<Range> mapped;

    while (!unmapped.empty()) {
      auto range = unmapped.front();
      unmapped.pop_front();
      bool is_mapped = false;
      for (const auto &mapping : mappings) {
        auto mapped_range = mapping.map(range);
        if (mapped_range) {
          mapped.push_back(mapped_range->mapped);
          if (mapped_range->prior) {
            unmapped.push_back(*(mapped_range->prior));
          }
          if (mapped_range->posterior) {
            unmapped.push_back(mapped_range->posterior.value());
          }
          is_mapped = true;
          break;
        }
      }
      if (is_mapped) {
        continue;
      }

      // cannot be mapped
      mapped.push_back(range);
    }

    return merge_all_ranges(mapped);
  }

  [[nodiscard]] static Section parse(std::ifstream &file) {
    std::vector<Mapping> mappings;
    std::string line;

    // discard section header
    std::getline(file, line);

    while (std::getline(file, line)) {
      if (line.empty()) {
        break;
      }
      mappings.push_back(Mapping::parse(line));
    }

    std::sort(mappings.begin(), mappings.end());

    return {.mappings = mappings};
  }
};
} // namespace stbf

std::ostream &operator<<(std::ostream &os, const stbf::Section &section) {
  for (const auto &mapping : section.mappings) {
    os << mapping << "\n";
  }

  return os;
}
