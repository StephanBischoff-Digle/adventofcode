#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <vector>

using u32 = uint32_t;

int main(int argc, char *argv[]) {
  // Read file
  if (argc == 1) {
    std::cout << "Requires input file" << std::endl;
    return 0;
  }

  std::ifstream infile(argv[1]);

  u32 accumulator = 0;
  std::string line;
  while (std::getline(infile, line)) {
    line.erase(std::remove_if(line.begin(), line.end(),
                              [](char c) { return !std::isdigit(c); }),
               line.end());
    std::string buffer({line.front(), line.back()});

    accumulator += std::stoi(buffer);
  }

  std::cout << accumulator << std::endl;

  return 0;
}
