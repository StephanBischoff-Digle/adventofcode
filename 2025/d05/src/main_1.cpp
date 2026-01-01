#include <charconv>
#include <fstream>
#include <iostream>
#include <optional>
#include <ranges>
#include <span>
#include <vector>

class Interval {
  public:
    static std::optional<Interval> from_string(std::string_view s) {
        auto parts = std::views::split(s, '-') | std::views::transform([](auto&& part) {
                         return std::string_view(&*part.begin(), static_cast<size_t>(std::ranges::distance(part)));
                     });

        std::size_t start{};
        std::size_t end{};
        auto        first = parts.front();
        auto        last  = *std::ranges::next(parts.begin(), 1);

        // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-pointer-arithmetic)
        auto r = std::from_chars(first.data(), first.data() + first.size(), start);
        if (r.ec != std::errc()) {
            return std::nullopt;
        }

        // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-pointer-arithmetic)
        r = std::from_chars(last.data(), last.data() + last.size(), end);
        if (r.ec != std::errc()) {
            return std::nullopt;
        }

        return Interval{ start, end };
    }

    Interval(size_t start, size_t end)
        : d_start{ start }
        , d_end{ end } {
    }

    [[nodiscard]] bool contains(size_t id) const {
        return id >= d_start && id <= d_end;
    }

  private:
    size_t d_start;
    size_t d_end;
};

bool is_fresh(const std::vector<Interval>& ivals, size_t v) {
    for (const auto ival : ivals) {
        if (ival.contains(v)) {
            return true;
        }
    }
    return false;
}

int main(int argc, char* argv[]) {
    std::span<char*> args(argv, static_cast<size_t>(argc));

    if (argc == 1) {
        std::cout << "Error: input file required!\n";
        return -1;
    }

    std::vector<Interval> ivals{};
    size_t acc {0};

    std::ifstream infile{ args[1] };
    std::string   line;
    bool          reading_intervals{ true };
    while (std::getline(infile, line)) {
        if (reading_intervals && !line.empty()) {
            auto ival = Interval::from_string(line);
            if (ival) {
                ivals.push_back(ival.value());
            } else {
                std::cerr << "Failed to parse interval\n";
                return -1;
            }
            continue;
        }

        if (line.empty()) {
            reading_intervals = false;
            std::cout << "parsing ivals done!\n";
            continue;
        }
        
        size_t id {0};
        // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-pointer-arithmetic)
        auto r = std::from_chars(line.data(), line.data() + line.size(), id);
        if (r.ec != std::errc()) {
            std::cerr << "Failed to parse id\n";
            return -1;
        }
        
        if (is_fresh(ivals, id)) {
            std::cout << id << " is fresh\n";
            acc += 1;
        }
    }

    std::cout << "Solution: " << acc << "\n";
}
