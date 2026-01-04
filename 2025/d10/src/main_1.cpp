#include <algorithm>
#include <cstdint>
#include <deque>
#include <fstream>
#include <iostream>
#include <ranges>
#include <span>
#include <string>
#include <string_view>
#include <vector>

class Problem {
    uint32_t              d_target_node{ 0 };
    std::vector<uint32_t> d_transitions;

  public:
    explicit Problem(std::string_view input) {
        // Find start of target node description
        size_t idx{ 0 };
        for (; input[idx] != '['; idx++) {
        }
        idx++;

        // Parse the target node
        for (size_t shift{ 0 }; input[idx] != ']'; idx++, shift++) {
            d_target_node += static_cast<uint32_t>((input[idx] == '#' ? 1 : 0) << shift);
        }

        // Parse the button connections
        auto button_split = input | std::views::split('(') | std ::views::drop(1);
        for (const auto& button : button_split) {
            uint32_t connections{ 0 };
            for (char c : button) {
                if (c == ',') {
                    continue;
                } else if (c == ')') {
                    break;
                }
                connections += 1 << (c - '0');
            }
            d_transitions.push_back(connections);
        }
    }

    /**
     * Calculates the number of button presses to get to the target output.
     *
     * Basically via Dijkstra's algorithm.
     */
    [[nodiscard]]
    uint32_t steps_to_solve() const {
        struct LengthToNode {
            uint32_t node;
            uint32_t length;
        };

        auto ltn_comparator = [](LengthToNode l, LengthToNode r) {
            return l.length < r.length;
        };

        std::deque<LengthToNode> unvisited;
        unvisited.emplace_back(0, 0);
        std::vector<uint32_t> visited;

        while (!unvisited.empty()) {
            auto current = unvisited.front();
            unvisited.pop_front();
            visited.push_back(current.node);

            // Done if we are at the target node.
            if (current.node == d_target_node) {
                return current.length;
            }

            bool unvisited_changed{ false };

            // Iterate over the transitions, generate or update the neighboring nodes.
            for (const uint32_t con : d_transitions) {
                uint32_t neighbor = current.node ^ con;

                // We already visited the neighbor
                if (std::ranges::contains(visited, neighbor)) {
                    continue;
                }

                // The neighbor was not visited before
                bool found{ false };
                for (size_t i{ 0 }; i < unvisited.size(); i++) {
                    if (unvisited[i].node == neighbor) {
                        unvisited[i].length = std::min(unvisited[i].length, current.length + 1);
                        found               = true;
                        break;
                    }
                }
                // Totaly new neighbor, insert it into the queue
                if (!found) {
                    unvisited.emplace_back(neighbor, current.length + 1);
                }
                unvisited_changed = true;
            }

            // sort if something happaned to the unvisited nodes.
            if (unvisited_changed) {
                std::ranges::sort(unvisited, ltn_comparator);
            }
        }

        return UINT32_MAX;
    }
};

int main(int argc, char* argv[]) {
    std::span<char*> args(argv, static_cast<size_t>(argc));

    if (args.size() == 1) {
        std::cout << "Error: input file required!\n";
        return -1;
    }

    std::ifstream infile{ args[1] };
    std::string   line;

    uint32_t steps{ 0 };

    // Read all the points
    while (std::getline(infile, line)) {
        Problem p(line);
        steps += p.steps_to_solve();
    }

    std::cout << "Solution: " << steps << "\n";
}
