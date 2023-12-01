#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <span>
#include <string>

using u32 = uint32_t;

int main(int argc, char *argv[]) {
  auto args = std::span(argv, size_t(argc));

  // Read file
  if (argc == 1) {
    std::cout << "Requires input file\n";
    return 0;
  }

  std::ifstream infile(args[1]);

  u32 accumulator = 0;
  std::string line;
  while (std::getline(infile, line)) {
    line.erase(std::remove_if(line.begin(), line.end(),
                              [](char c) { return !std::isdigit(c); }),
               line.end());
    const std::string buffer({line.front(), line.back()});

    accumulator += std::stoi(buffer);
  }

  std::cout << accumulator << "\n";

  return 0;
}
