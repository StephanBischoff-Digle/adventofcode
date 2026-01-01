#include <algorithm>
#include <charconv>
#include <fstream>
#include <iostream>
#include <optional>
#include <ranges>
#include <span>
#include <vector>

class Interval {
  public:
    size_t d_start;
    size_t d_end;

    // This is just parsing logic
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
};

int main(int argc, char* argv[]) {
    std::span<char*> args(argv, static_cast<size_t>(argc));

    if (argc == 1) {
        std::cout << "Error: input file required!\n";
        return -1;
    }

    std::vector<Interval> ivals{};

    std::ifstream infile{ args[1] };
    std::string   line;
    while (std::getline(infile, line)) {
        if (line.empty()) {
            break;
        }

        auto ival = Interval::from_string(line);
        if (ival) {
            ivals.push_back(ival.value());
        } else {
            std::cerr << "Failed to parse interval\n";
            return -1;
        }
    }

    // sort the intervals by their start values.
    std::ranges::sort(ivals, {}, &Interval::d_start);

    // we can merge if the start of the current interval is within the merge interval.
    // if we cannot merge, we calculate the containing ID's and
    // set the new merge interval to the current interval.
    size_t merge_idx{ 0 };
    size_t acc{ 0 };
    for (size_t i{ 1 }; i < ivals.size(); i++) {
        if (ivals[merge_idx].contains(ivals[i].d_start)) {
            ivals[merge_idx].d_end = std::max(ivals[merge_idx].d_end, ivals[i].d_end);
        } else {
            // we need to add one, because the interval is inclusive.
            acc += ivals[merge_idx].d_end - ivals[merge_idx].d_start + 1;
            merge_idx = i;
        }
    }
    // count the last interval.
    acc += ivals[merge_idx].d_end - ivals[merge_idx].d_start + 1;

    std::cout << "Solution: " << acc << "\n";
}
