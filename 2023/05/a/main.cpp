#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <optional>
#include <span>
#include <unordered_set>
#include <vector>

using u32 = std::uint32_t;

/**
 * Parses a string of space separated u32 into a set of those numbers.
 * @attention The string must not have spaces after the last number.
 * @param list The space separated string of u32 numbers.
 * @return The set of the parsed numbers.
 */
[[nodiscard]] std::unordered_set<u32> parse_list(std::string list) {
  std::unordered_set<u32> set;

  size_t pos = 0;
  while ((pos = list.find(' ')) != std::string::npos) {
    // This checks for leading whitespaces. If a leading space is found, the
    // parsing is skipped and the space character is erased.
    if (pos != 0) {
      set.insert(std::stol(list.substr(0, pos)));
    }
    list.erase(0, pos + 1);
  }
  // Parse the last number.
  set.insert(std::stol(list.substr(0, pos)));

  return set;
}

struct Mapping {
  u32 source_start;
  u32 destination_start;
  u32 range;

  [[nodiscard]] std::optional<u32> map(u32 index) const {
    if (index >= source_start && index <= (source_start + range)) {
      return destination_start + (index - source_start);
    }
    return std::nullopt;
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

    return {.source_start = s, .destination_start = d, .range = r};
  }
};

struct Section {
  std::vector<Mapping> mappings;

  [[nodiscard]] u32 map(u32 index) const {
    for (const auto &mapping : mappings) {
      const auto v = mapping.map(index);
      if (v) {
        return *v;
      }
    }
    return index;
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
    return {.mappings = mappings};
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
  std::string line;
  std::getline(infile, line);
  const auto pos = line.find(':') + 2;

  std::unordered_set<u32> seeds = parse_list(line.substr(pos));

  // discard empty line
  std::getline(infile, line);

  const auto seed_to_soil = Section::parse(infile);
  const auto soil_to_fertilizer = Section::parse(infile);
  const auto fertilizer_to_water = Section::parse(infile);
  const auto water_to_light = Section::parse(infile);
  const auto light_to_temperature = Section::parse(infile);
  const auto temperature_to_humidity = Section::parse(infile);
  const auto humidity_to_location = Section::parse(infile);

  u32 min_loc = -1;
  for (const auto &seed : seeds) {
    const auto soil = seed_to_soil.map(seed);
    const auto fertilizer = soil_to_fertilizer.map(soil);
    const auto water = fertilizer_to_water.map(fertilizer);
    const auto light = water_to_light.map(water);
    const auto temperature = light_to_temperature.map(light);
    const auto humidity = temperature_to_humidity.map(temperature);
    const auto location = humidity_to_location.map(humidity);
    min_loc = std::min(min_loc, location);
  }

  std::cout << min_loc << '\n';

  return 0;
}
