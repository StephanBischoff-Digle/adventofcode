#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <numeric>
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

int main(int argc, char *argv[]) {
  auto args = std::span(argv, size_t(argc));

  // Read file
  if (argc == 1) {
    std::cout << "Requires input file\n";
    return 0;
  }

  std::ifstream infile_line_count(args[1]);

  // pre-allocate with number of lines, init to 1
  std::vector<u32> copies(
      std::count(std::istreambuf_iterator<char>(infile_line_count),
                 std::istreambuf_iterator<char>(), '\n'),
      1);

  std::ifstream infile(args[1]);

  std::string line;
  size_t game = 0;
  while (std::getline(infile, line)) {
    const auto winning_start = line.find(':') + 2;
    const auto winning_end = line.find('|') - 1;
    auto winning = line.substr(winning_start, winning_end - winning_start);
    auto owned =
        line.substr(winning_end + 3, line.length() - (winning_end + 3));

    auto winning_numbers = parse_list(winning);
    auto owned_numbers = parse_list(owned);
    u32 count = 0;
    for (const auto &n : owned_numbers) {
      if (winning_numbers.contains(n)) {
        count++;
      }
    }

    // add the number of coppies of the current game to the next `count` cards.
    const auto my_count = copies[game];
    for (auto idx = 0; idx < count; idx++) {
      copies[game + idx + 1] += my_count;
    }
    game += 1;
  }

  // reduce defaults to sum
  std::cout << std::reduce(copies.begin(), copies.end()) << '\n';

  return 0;
}
