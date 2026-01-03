#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <ranges>
#include <span>
#include <string>
#include <string_view>
#include <vector>

#include "stbi/common.hpp"

struct Point {
    uint64_t x;
    uint64_t y;
    uint64_t z;

    [[nodiscard]]
    uint64_t squared_dist(const Point& other) const {
        auto dx = stbi::udelta(other.x, x);
        auto dy = stbi::udelta(other.y, y);
        auto dz = stbi::udelta(other.z, z);
        return dx * dx + dy * dy + dz * dz;
    }

    explicit Point(std::string_view sv) {
        size_t cnt{ 0 };
        auto   parts = sv | std::views::split(',');
        for (const auto&& part : parts) {
            auto m_v = stbi::parse_int<uint64_t>({ part.begin(), static_cast<size_t>(std::ranges::distance(part)) });
            switch (cnt) {
            case 0:
                x = m_v.value();
                break;
            case 1:
                y = m_v.value();
                break;
            case 2:
                z = m_v.value();
                break;
            default:
                return;
                break;
            }
            cnt++;
        }
    }
};

/**
 * Struct to hold the distance between two points.
 *
 * The points are identified by `i` and `j` in the context of a Point array.
 */
struct DistIdx {
    uint64_t dist;  //!< The distance between the points.
    size_t   i;     //!< The index of a point inside a Point array.
    size_t   j;     //!< The index of a point inside a Point array.
};

/**
 * Merges two clusters `from` to `to`.
 *
 * @param cluster_list The cluster map. The index referes to a point, the value to the cluster-id.
 * @param from The cluster-id that should be replaced by `to`.
 * @param to The cluster-id that replaces `from`.
 */
void cleanup_cluster(std::vector<uint32_t>& cluster_list, uint32_t from, uint32_t to) {
    if (from == to) {
        return;
    }
    for (size_t i{ 0 }; i < cluster_list.size(); i++) {
        if (cluster_list[i] == from) {
            cluster_list[i] = to;
        }
    }
}

int main(int argc, char* argv[]) {
    std::span<char*> args(argv, static_cast<size_t>(argc));

    if (args.size() == 1) {
        std::cout << "Error: input file required!\n";
        return -1;
    }

    std::vector<Point>   points;
    std::vector<DistIdx> distance_stack;

    std::ifstream infile{ args[1] };
    std::string   line;

    // Read all the points
    while (std::getline(infile, line)) {
        points.emplace_back(line);
    }

    // Calculate all the distances
    distance_stack.reserve((points.size() * points.size()) / 2);
    for (size_t i{ 0 }; i < points.size(); i++) {
        for (size_t j{ i + 1 }; j < points.size(); j++) {
            distance_stack.emplace_back(points[i].squared_dist(points[j]), i, j);
        }
    }

    // Sort the distance stack to find smallest distances first.
    std::ranges::sort(distance_stack, {}, &DistIdx::dist);

    // Cluster-ID map
    std::vector<uint32_t> cluster_ids(points.size(), UINT32_MAX);

    // Current ID for a new cluster.
    uint32_t cluster_n{ 0 };
    uint64_t solution{ 0 };
    for (const auto& d : distance_stack) {
        // Select smallest cluster-id for potential merge
        uint32_t cluster = std::min(cluster_ids[d.i], cluster_ids[d.j]);
        if (cluster == UINT32_MAX) {
            // creating a new cluster
            cluster_ids[d.i] = cluster_n;
            cluster_ids[d.j] = cluster_n;
            cluster_n++;
        } else {
            // add unassigned point to cluster
            if (cluster_ids[d.i] == UINT32_MAX) {
                cluster_ids[d.i] = cluster;

                // solution = points[d.i].x * points[d.j].x;
            } else if (cluster_ids[d.j] == UINT32_MAX) {
                cluster_ids[d.j] = cluster;

                // solution = points[d.i].x * points[d.j].x;
            } else {
                // merge two clusters.
                if (cluster_ids[d.i] != cluster_ids[d.j]) {
                    auto from = std::max(cluster_ids[d.i], cluster_ids[d.j]);
                    cleanup_cluster(cluster_ids, from, cluster);

                    // solution = points[d.i].x * points[d.j].x;
                }
            }
        }

        // NOTE: This may be slower than just going through the
        //       distance_stack and calculating the solutions on merge.
        if (std::ranges::all_of(cluster_ids, [cluster](auto v) {
                return v == cluster;
            })) {
            // Done, all belong to the same cluster.
            solution = points[d.i].x * points[d.j].x;
            break;
        }
    }

    std::cout << "Solution: " << solution << "\n";
}
