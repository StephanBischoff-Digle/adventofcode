#include <algorithm>
#include <charconv>
#include <fstream>
#include <functional>
#include <iostream>
#include <optional>
#include <span>
#include <sstream>
#include <vector>

using std::multiplies;

std::optional<uint64_t> parse_int(std::string_view s) {
    uint64_t x{ 0 };
    // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-pointer-arithmetic)
    auto r = std::from_chars(s.data(), s.data() + s.size(), x);
    if (r.ec != std::errc()) {
        std::cerr << "Failed to parse id\n";
        return std::nullopt;
    }
    return x;
}

int main(int argc, char* argv[]) {
    std::span<char*> args(argv, static_cast<size_t>(argc));

    if (argc == 1) {
        std::cout << "Error: input file required!\n";
        return -1;
    }

    std::vector<std::vector<uint64_t>> operands;
    uint64_t                           acc{ 0 };

    std::ifstream infile{ args[1] };
    std::string   line;
    bool          first_line{ true };
    while (std::getline(infile, line)) {
        bool is_operation{ false };
        if (line.starts_with('*') || line.starts_with('+')) {
            is_operation = true;
        }

        std::istringstream iss(line);
        std::string        word;
        size_t             i{ 0 };
        while (iss >> word) {
            if (first_line) {
                operands.emplace_back();
            }
            if (!is_operation) {
                auto v = parse_int(word);
                if (v) {
                    operands[i].push_back(v.value());
                } else {
                    return -1;
                }
            } else {
                if (word == "*") {
                    acc += std::ranges::fold_left(operands[i], 1, std::multiplies<>());
                } else if (word == "+") {
                    acc += std::ranges::fold_left(operands[i], 0, std::plus<>());
                } else {
                    std::cerr << "Found non-operation: " << word << " aborting!\n";
                    return -1;
                }
            }
            i++;
        }
        first_line = false;
    }

    std::cout << "Solution: " << acc << "\n";
}
