#include <cstdint>
#include <fstream>
#include <iostream>
#include <span>
#include <string>
#include <vector>

#include "Range.hpp"
#include "Section.hpp"

using u32 = std::uint32_t;

void print_ranges(const std::string &title,
                  const std::vector<stbf::Range> &ranges) {
  std::cout << "==============================\n";
  std::cout << title << "\n";
  for (const auto &range : ranges) {
    std::cout << range << "\n";
  }
}

void print_section(const std::string &title, const stbf::Section &section) {
  std::cout << "==============================\n";
  std::cout << title << "\n";
  std::cout << section << "\n\n";
}

[[nodiscard]] std::vector<stbf::Range> parse_list(std::string list) {
  std::vector<stbf::Range> ranges;

  size_t pos = 0;
  while ((pos = list.find(' ')) != std::string::npos) {
    u32 start = 0;
    u32 length = 0;
    // This checks for leading whitespaces. If a leading space is found, the
    // parsing is skipped and the space character is erased.
    if (pos != 0) {
      start = std::stol(list.substr(0, pos));
    }
    list.erase(0, pos + 1);

    while (true) {
      pos = list.find(' ');
      if (pos != 0) {
        length = std::stol(list.substr(0, pos));
        list.erase(0, pos + 1);
        break;
      }
      list.erase(0, pos + 1);
    }
    ranges.emplace_back(start, length - 1);
  }

  return ranges;
}

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

  // Get the seeds
  const auto seeds = parse_list(line.substr(pos));

  // discard empty line
  std::getline(infile, line);

  //////////////////////////////////////////////////////////////////////////////////
  // Do the parsing
  const auto seed_to_soil = stbf::Section::parse(infile);
  print_section(std::string{"seed_to_soil"}, seed_to_soil);

  const auto soil_to_fertilizer = stbf::Section::parse(infile);
  print_section(std::string{"soil_to_fertilizer"}, soil_to_fertilizer);

  const auto fertilizer_to_water = stbf::Section::parse(infile);
  print_section(std::string{"fertilizer_to_water"}, fertilizer_to_water);

  const auto water_to_light = stbf::Section::parse(infile);
  print_section(std::string{"water_to_light"}, water_to_light);

  const auto light_to_temperature = stbf::Section::parse(infile);
  print_section(std::string{"light_to_temperature"}, light_to_temperature);

  const auto temperature_to_humidity = stbf::Section::parse(infile);
  print_section(std::string{"temperature_to_humidity"},
                temperature_to_humidity);

  const auto humidity_to_location = stbf::Section::parse(infile);
  print_section(std::string{"humidity_to_location"}, humidity_to_location);

  //////////////////////////////////////////////////////////////////////////////////
  // Do the mapping
  print_ranges(std::string{"seeds"}, seeds);

  const auto soils = seed_to_soil.map(seeds);
  print_ranges(std::string{"soils"}, soils);

  const auto fertilizers = soil_to_fertilizer.map(soils);
  print_ranges(std::string{"fertilizers"}, fertilizers);

  const auto waters = fertilizer_to_water.map(fertilizers);
  print_ranges(std::string{"waters"}, waters);

  const auto lights = water_to_light.map(waters);
  print_ranges(std::string{"lights"}, lights);

  const auto temperatures = light_to_temperature.map(lights);
  print_ranges(std::string{"temperatures"}, temperatures);

  const auto humidities = temperature_to_humidity.map(temperatures);
  print_ranges(std::string{"humidities"}, humidities);

  auto locations = humidity_to_location.map(humidities);
  print_ranges(std::string{"locations"}, locations);

  std::sort(locations.begin(), locations.end());
  std::cout << "\nAnswer: " << locations.front().start << "\n";

  return 0;
}
