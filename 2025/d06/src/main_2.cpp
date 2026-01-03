#include <algorithm>
#include <charconv>
#include <fstream>
#include <functional>
#include <iomanip>
#include <iostream>
#include <optional>
#include <span>
#include <vector>

using std::multiplies;

enum class Operation : uint8_t {
    ADDITION,
    MULTIPLICATION,
    NONE,
};

[[nodiscard]]
std::optional<uint64_t> parse_int(std::string_view s) {
    uint64_t x{ 0 };
    // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-pointer-arithmetic)
    auto r = std::from_chars(s.data(), s.data() + s.size(), x);
    if (r.ec != std::errc()) {
        if (r.ec == std::errc::invalid_argument) {
            std::cerr << "This is not a number.\n";
        } else if (r.ec == std::errc::result_out_of_range) {
            std::cerr << "This number is larger than an int.\n";
        }
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

    std::ifstream            infile{ args[1] };
    std::string              line;
    std::vector<std::string> file;

    // Read in all the lines
    while (std::getline(infile, line)) {
        file.push_back(line);
    }

    uint64_t global_acc{ 0 };
    uint64_t local_acc{ 0 };

    bool      is_first_pass{ true };
    size_t    n_cols{ file[0].length() };
    Operation operation{ Operation::NONE };

    for (size_t col{ 0 }; col < n_cols; col++) {
        std::string number_string{};

        // assemble the column string
        for (size_t row{ 0 }; row < file.size(); row++) {
            // find the max number of cols
            if (is_first_pass) {
                n_cols = std::max(n_cols, file[row].length());
            }

            if (col >= file[row].length()) {
                number_string += " ";
                continue;
            }

            char current = file[row][col];
            switch (current) {
            case '+':
                operation = Operation::ADDITION;
                local_acc = 0;  // additive identity
                break;
            case '*':
                operation = Operation::MULTIPLICATION;
                local_acc = 1;  // multiplicative identity
                break;
            case ' ':
                break;
            default:
                number_string += current;
                break;
            }
        }

        if (number_string.empty()) {
            global_acc += local_acc;
            operation = Operation::NONE;
        } else {
            auto n = parse_int(number_string);
            if (n) {
                switch (operation) {
                case Operation::ADDITION:
                    local_acc += n.value();
                    break;
                case Operation::MULTIPLICATION:
                    local_acc *= n.value();
                    break;
                default:
                    std::cerr << "NONE operation, somehow\n";
                    return -1;
                }
            } else {
                std::cerr << "Failed to parse " << std::quoted(number_string) << " on column " << col << "\n";
                return -1;
            }
        }
    }
    // there is no last empty column, so we have to add up here.
    global_acc += local_acc;

    std::cout << "Solution: " << global_acc << "\n";
}
