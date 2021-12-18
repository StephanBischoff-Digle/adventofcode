#!/usr/bin/env python3

import fileinput
import logging as log
from math import sqrt, ceil


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


def parse_input(in_str: str) -> tuple[tuple[int, int], tuple[int, int]]:
    s0 = in_str.split(": x=")[1]
    xy = s0.split(", y=")
    sx = xy[0].split("..")
    xs = (int(sx[0]), int(sx[1]))

    sy = xy[1].split("..")
    ys = (int(sy[0]), int(sy[1]))

    return (xs, ys)


def check_impact(x, y, area) -> bool:
    x_in = min(area[0]) <= x <= max(area[0])
    y_in = min(area[1]) <= y <= max(area[1])
    return x_in and y_in


def check_gone(x, y, area) -> bool:
    return x > max(area[0]) or y < min(area[1])


def check_trajectory(dx, dy, area) -> bool:
    x = 0
    y = 0
    while not check_impact(x, y, area):
        x += dx
        y += dy
        if dx > 0:
            dx -= 1
        dy -= 1
        if check_gone(x, y, area):
            return False
    return True


def main() -> None:
    in_lst = read_input()
    area = parse_input(in_lst[0])

    # lower bound (dx > 0 at min(area_x))
    dx0 = ceil(0.5 * (sqrt(8 * min(area[0]) + 1) - 1))

    n_hits = 0

    max_checks = (max(area[0]) + 1 - dx0) * (abs(min(area[1])) - min(area[1]))
    i = 0
    for dx in range(dx0, max(area[0]) + 1):
        for dy in range(min(area[1]), abs(min(area[1]))):
            n_hits += 1 if check_trajectory(dx, dy, area) else 0
            i += 1
            print(
                "\t{:3d}% | {:10d} of {:10d}".format(
                    (100 * i) // max_checks, i, max_checks
                ),
                end="\r",
            )

    print()
    print(n_hits)


if __name__ == "__main__":
    main()
