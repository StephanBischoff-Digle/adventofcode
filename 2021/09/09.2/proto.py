#!/usr/bin/env python3

import fileinput
from functools import reduce
import operator


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


def check_bounds(
    p: tuple[int, int], bounds: tuple[tuple[int, int], tuple[int, int]]
) -> bool:
    ((x_min, x_max), (y_min, y_max)) = bounds
    (x, y) = p
    return x_min <= x <= x_max and y_min <= y <= y_max


def kernel(pos: tuple[int, int], heights: list[list[int]]) -> list[int]:
    (x, y) = pos
    bounds = ((0, len(heights[y]) - 1), (0, len(heights) - 1))
    adj = list(
        filter(
            lambda p: check_bounds(p, bounds),
            [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)],
        )
    )

    hs = [heights[y][x] for (x, y) in adj]
    return hs


def measure_basin_size(p: tuple[int, int], heights: list[list[int]]) -> int:
    count = 0
    marker = -1
    (x, y) = p
    bounds = ((0, len(heights[y]) - 1), (0, len(heights) - 1))
    queue = [p]
    heights[y][x] = marker

    def handle_pos(t_x: int, t_y: int) -> None:
        if check_bounds((t_x, t_y), bounds):
            if heights[t_y][t_x] not in [marker, 9]:
                heights[t_y][t_x] = marker
                queue.append((t_x, t_y))

    while queue:
        count += 1
        (cx, cy) = queue.pop()
        handle_pos(cx + 1, cy)
        handle_pos(cx - 1, cy)
        handle_pos(cx, cy + 1)
        handle_pos(cx, cy - 1)

    return count


def main() -> None:
    in_lst = read_input()
    heights = [[int(h) for h in line] for line in in_lst]

    mins = []
    for y in range(len(heights)):
        for x in range(len(heights[y])):
            v = heights[y][x]
            if all(map(lambda a: a > v, kernel((x, y), heights))):
                mins.append((x, y))

    sizes = [measure_basin_size(m, heights) for m in mins]
    sizes.sort(reverse=True)

    print(reduce(operator.mul, sizes[0:3]))


if __name__ == "__main__":
    main()
