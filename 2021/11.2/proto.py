#!/usr/bin/env python3

import fileinput
from collections import deque


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


def print_field(energies: list[list[int]]) -> None:
    for y in range(len(energies)):
        print(energies[y])
    print()


def get_neighbors(x: int, y: int, max_x: int, max_y: int) -> list[tuple[int, int]]:
    ns = []
    for cx in range(max(x - 1, 0), min(x + 2, max_x)):
        for cy in range(max(y - 1, 0), min(y + 2, max_y)):
            if cx == x and cy == y:
                continue
            ns.append((cx, cy))

    return ns


def step(energies: list[list[int]]) -> int:
    update_q = deque()
    flashed = set()

    # update energies
    for y in range(len(energies)):
        for x in range(len(energies[y])):
            if (x, y) not in flashed:
                energies[y][x] = (energies[y][x] + 1) % 10
                if energies[y][x] == 0:
                    flashed.add((x, y))
                    [
                        update_q.append(p)
                        for p in get_neighbors(x, y, len(energies[y]), len(energies))
                    ]
    # handle update_q for cascading
    while update_q:
        (x, y) = update_q.pop()
        if (x, y) not in flashed:
            energies[y][x] = (energies[y][x] + 1) % 10
            if energies[y][x] == 0:
                flashed.add((x, y))
                [
                    update_q.append(p)
                    for p in get_neighbors(x, y, len(energies[y]), len(energies))
                ]

    return len(flashed)


def main() -> None:
    in_lst = read_input()
    energies = [[int(h) for h in line] for line in in_lst]
    n = len(energies) * len(energies[0])
    i = 1
    while step(energies) != n:
        i += 1
    print(i)


if __name__ == "__main__":
    main()
