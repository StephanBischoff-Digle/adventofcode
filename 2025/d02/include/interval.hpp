#pragma once

#include <cstddef>

namespace advent {
    class Interval {
      public:
        Interval(std::size_t start, std::size_t end)
            : d_start{ start }
            , d_end{ end } {
        }

        [[nodiscard]] bool is_inside(std::size_t x) const noexcept {
            return x >= d_start && x <= d_end;
        }

        std::size_t d_start;
        std::size_t d_end;
    };
}  // namespace advent
