#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <ranges>
#include <span>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

#include "stbi/common.hpp"

struct Point {
    int64_t x;
    int64_t y;

    /**
     * Calculate the area of the bounding box of the two points
     */
    [[nodiscard]]
    int64_t area(const Point& other) const {
        auto dx = stbi::udelta(other.x, x) + 1;
        auto dy = stbi::udelta(other.y, y) + 1;
        return dx * dy;
    }

    Point(int64_t x, int64_t y)
        : x{ x }
        , y{ y } {
    }

    explicit Point(std::string_view sv) {
        size_t cnt{ 0 };
        auto   parts = sv | std::views::split(',');
        for (const auto&& part : parts) {
            auto m_v = stbi::parse_int<int64_t>(
                { part.begin(), static_cast<size_t>(std::ranges::distance(part)) });
            switch (cnt) {
            case 0:
                x = m_v.value();
                break;
            case 1:
                y = m_v.value();
                break;
            default:
                return;
                break;
            }
            cnt++;
        }
    }
};

/// Alias for the Line, could be a struct, but meh …
using Line = std::pair<Point, Point>;

/**
 * This tests if the given point is on the line segment.
 */
bool point_on_segment(const Line& segment, const Point& p) {
    auto [sa, sb] = segment;
    int64_t cross = (p.y - sa.y) - (sb.x - sa.x) - (p.x - sa.x) * (sb.y - sa.y);

    // The point is not on the unbounded line of the segment
    if (cross != 0) {
        return false;
    }

    // Check if the point is withing the bounds of the segment
    auto min_x = std::min(sa.x, sb.x);
    auto max_x = std::max(sa.x, sb.x);
    auto min_y = std::min(sa.y, sb.y);
    auto max_y = std::max(sa.y, sb.y);
    if (min_x <= p.x && p.x <= max_x && min_y <= p.y && p.y <= max_y) {
        return true;
    }

    return false;
}

/**
 * Tests, wheter a point is directly on top of the polygons border.
 */
bool point_on_poly_border(std::span<const Line> poly, const Point& p) {
    for (const auto& segment : poly) {
        if (point_on_segment(segment, p)) {
            return true;
        }
    }
    return false;
}

/**
 * Tests wheter the given segment is vertical.
 */
bool is_vertical(const Line& segment) {
    auto [a, b] = segment;
    return a.x == b.x;
}

/**
 * Tests wheter the given segment is horizontal.
 */
bool is_horizontal(const Line& segment) {
    auto [a, b] = segment;
    return a.y == b.y;
}

/**
 * Tests wheter a point is within the polygon.
 *
 * This uses the standard ray-casting along the x-axis.
 * If the emitted ray crosses the border an odd number of times, the point is within the polygon.
 */
bool point_inside_poly(std::span<const Line> poly, const Point& p) {
    int intersections{ 0 };

    for (const auto& segment : poly) {
        if (is_vertical(segment)) {
            int64_t seg_x     = segment.first.x;
            int64_t seg_y_min = std::min(segment.first.y, segment.second.y);
            int64_t seg_y_max = std::max(segment.first.y, segment.second.y);

            if (seg_x > p.x && p.y >= seg_y_min && p.y < seg_y_max) {
                intersections += 1;
            }
        }
    }

    return (intersections % 2 != 0);
}

/**
 * Tests wheter two line segments intersect.
 */
bool segments_intersect(const Line& a, const Line& b) {
    // if the segments orientation is the same, they cannot intersect.
    if ((is_horizontal(a) && is_horizontal(b)) || (is_vertical(a) && is_vertical(b))) {
        return false;
    }

    auto hori = (is_horizontal(a) ? a : b);
    auto vert = (is_vertical(a) ? a : b);

    int64_t hori_x_min = std::min(hori.first.x, hori.second.x);
    int64_t hori_x_max = std::max(hori.first.x, hori.second.x);
    int64_t vert_y_min = std::min(vert.first.y, vert.second.y);
    int64_t vert_y_max = std::max(vert.first.y, vert.second.y);
    return (vert.first.x > hori_x_min && vert.first.x < hori_x_max) &&
           (hori.first.y > vert_y_min && hori.first.y < vert_y_max);
}

/**
 * Tests whether the line segment intersect the polygons border somewhere.
 */
bool segment_intersects_poly_border(std::span<const Line> poly, const Line& segment) {
    for (const auto& border : poly) {
        if (segments_intersect(border, segment)) {
            return true;
        }
    }
    return false;
}

/**
 * Tests wheter the inner polygon is contained within the outer polygon.
 */
bool is_contained(std::span<const Line> outer, std::span<const Line> inner) {
    for (const auto& segment : inner) {
        auto [p, _]          = segment;
        bool point_inside    = point_inside_poly(outer, p);
        bool point_on_border = point_on_poly_border(outer, p);

        if (!point_inside && !point_on_border) {
            return false;
        }

        bool segment_intersects = segment_intersects_poly_border(outer, segment);
        if (segment_intersects) {
            return false;
        }
    }

    return true;
}

/**
 * Constructs the bounding box of the two points.
 */
std::array<Line, 4> construct_inner_polygon(const Point& a, const Point& b) {
    Point ab(a.x, b.y);
    Point ba(b.x, a.y);
    return { std::make_pair(a, ab),
             std::make_pair(ab, b),
             std::make_pair(b, ba),
             std::make_pair(ba, a) };
}

///////////////////////////////////////////////////////////////////////////////
int main(int argc, char* argv[]) {
    std::span<char*> args(argv, static_cast<size_t>(argc));

    if (args.size() == 1) {
        std::cout << "Error: input file required!\n";
        return -1;
    }

    std::vector<Point> points;

    std::ifstream infile{ args[1] };
    std::string   line;

    // Read all the points
    while (std::getline(infile, line)) {
        points.emplace_back(line);
    }

    // construct the polygon
    std::vector<Line> polygon = std::views::adjacent<2>(points) | std::ranges::to<std::vector>();
    polygon.emplace_back(points.back(), points.front());

    int64_t max_area{ 0 };
    for (size_t i{ 0 }; i < points.size(); i++) {
        for (size_t j{ i + 1 }; j < points.size(); j++) {
            // We filter by checking, if the bounding box of the candidate pair is
            // within the polygon.
            auto inner     = construct_inner_polygon(points[i], points[j]);
            auto contained = is_contained(polygon, inner);

            // Only concider if the inner is contained in the polygon
            if (contained) {
                auto area = points[i].area(points[j]);
                max_area  = std::max(max_area, area);
            }
        }
    }

    std::cout << "Solution: " << max_area << "\n";
}
