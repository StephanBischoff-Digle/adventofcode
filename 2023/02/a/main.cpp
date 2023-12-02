#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <span>
#include <string>
#include <vector>

using u32 = uint32_t;

/// Holds data regarding the observed numbers of colored cubes.
struct SetOfCubes {
  u32 r = 0; //!< Number of red cubes.
  u32 g = 0; //!< Number of green cubes.
  u32 b = 0; //!< Number of blue cubes.
};

/// Holds the data of a game.
struct Game {
  u32 id;                       //!< The games id.
  std::vector<SetOfCubes> sets; //!< The sets of cubes observed.
};

/// The constraint for a valid set of cubes (maximum)
constexpr SetOfCubes constraint{.r = 12, .g = 13, .b = 14};

/**
 * Checks if the arguments adheres to the constrains.
 * @param soc The set of cubes to check.
 * @return Wheter the argument is within the constraints.
 */
constexpr bool is_valid_soc(const SetOfCubes &soc) {
  return soc.r <= constraint.r && soc.g <= constraint.g &&
         soc.b <= constraint.b;
}

/**
 * Checks wheter the game is valid. All observed sets in the game must be valid.
 * @param game The game to check.
 * @return Wheter the game is valid.
 */
constexpr bool is_valid(const Game &game) {
  return std::all_of(game.sets.cbegin(), game.sets.cend(), is_valid_soc);
}

Game parse_line(std::string &&line) {
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
    auto g = parse_line(std::move(line));
    // std::cout << g.id << "\n";
    // for (const auto &soc : g.sets) {
    //   std::cout << "   {r: " << soc.r << ", g: " << soc.g << ", b: " << soc.b
    //             << "}\n";
    // }
    if (is_valid(g)) {
      accu += g.id;
    }
  }

  std::cout << accu << "\n";

  return 0;
}
