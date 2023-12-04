#include <cstdint>
#include <fstream>
#include <iostream>
#include <span>
#include <unordered_set>

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

int main(int argc, char *argv[]) {
  auto args = std::span(argv, size_t(argc));

  // Read file
  if (argc == 1) {
    std::cout << "Requires input file\n";
    return 0;
  }

  std::ifstream infile(args[1]);

  std::string line;
  u32 acc = 0;
  while (std::getline(infile, line)) {

    const auto winning_start = line.find(':') + 2;
    const auto winning_end = line.find('|') - 1;
    auto winning = line.substr(winning_start, winning_end - winning_start);
    auto owned =
        line.substr(winning_end + 3, line.length() - (winning_end + 3));

    auto winning_numbers = parse_list(winning);
    auto owned_numbers = parse_list(owned);
    u32 points = 0;
    for (const auto &n : owned_numbers) {
      if (winning_numbers.contains(n)) {
        points = points == 0 ? 1 : points * 2;
      }
    }
    acc += points;
  }

  std::cout << acc << "\n";

  return 0;
}
