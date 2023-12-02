#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <numeric>
#include <span>
#include <string>
#include <vector>

using u32 = uint32_t;

/// Holds data regarding the observed numbers of colored cubes.
struct SetOfCubes {
  u32 r = 0; //!< Number of red cubes.
  u32 g = 0; //!< Number of green cubes.
  u32 b = 0; //!< Number of blue cubes.

  /**
   * Calculates the power of the set.
   * @return The power of the set.
   */
  [[nodiscard]] u32 power() { return r * g * b; }
};

/// Holds the data of a game.
struct Game {
  u32 id;                       //!< The games id.
  std::vector<SetOfCubes> sets; //!< The sets of cubes observed.
};

/**
 * Constructs the set of cubes that whould make the game valid.
 * @param game The game for which the minimum set should be calculated.
 * @return The SetOfCubes that the game requires to be valid.
 */
[[nodiscard]] SetOfCubes minimum_set(const Game &game) {
  using std::max;
  return std::accumulate(
      game.sets.begin(), game.sets.end(), SetOfCubes{},
      [](const SetOfCubes &a, const SetOfCubes &b) -> SetOfCubes {
        return {.r = max(a.r, b.r), .g = max(a.g, b.g), .b = max(a.b, b.b)};
      });
}

/**
 * Parses the given line into a Game.
 * @param line The line that should be parsed.
 * @return The parsed Game.
 */
[[nodiscard]] Game parse_line(std::string &&line) {
  auto l = std::move(line);

  auto colon = l.find(':');
  auto id_str = l.substr(5, colon);

  Game game{.id = static_cast<u32>(std::stoul(id_str))};

  // parse all the sets
  size_t start = colon + 2;
  while (true) {
    size_t set_end = l.find(';', start);
    // std::cout << l.substr(start, end - start) << "\n";

    // parse the set
    size_t comma = start;
    SetOfCubes soc;
    while (true) {
      auto cube_end = std::min(l.find(',', comma), set_end);

      auto cubes = l.substr(comma, cube_end - comma);
      // std::cout << cubes << "\n";
      auto red = cubes.find("red");
      auto green = cubes.find("green");
      auto blue = cubes.find("blue");
      if (red != std::string::npos) {
        soc.r = static_cast<u32>(std::stoul(cubes.substr(0, red)));
      } else if (green != std::string::npos) {
        soc.g = static_cast<u32>(std::stoul(cubes.substr(0, green)));
      } else {
        soc.b = static_cast<u32>(std::stoul(cubes.substr(0, blue)));
      }

      if (cube_end == std::string::npos || cube_end == set_end) {
        break;
      }

      comma = cube_end + 1;
    }
    game.sets.push_back(soc);

    if (set_end == std::string::npos) {
      break;
    }
    start = set_end + 1;
  }

  return game;
}

int main(int argc, char *argv[]) {
  auto args = std::span(argv, size_t(argc));

  // Read file
  if (argc == 1) {
    std::cout << "Requires input file\n";
    return 0;
  }

  std::ifstream infile(args[1]);

  u32 accu = 0;

  std::string line;
  while (std::getline(infile, line)) {
    const auto g = parse_line(std::move(line));
    // std::cout << g.id << "\n";
    // for (const auto &soc : g.sets) {
    //   std::cout << "   {r: " << soc.r << ", g: " << soc.g << ", b: " <<
    //   soc.b
    //             << "}\n";
    // }
    accu += minimum_set(g).power();
  }

  std::cout << accu << "\n";

  return 0;
}
