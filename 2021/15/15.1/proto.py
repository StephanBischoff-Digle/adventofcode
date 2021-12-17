#!/usr/bin/env python3

import fileinput
from dataclasses import dataclass
from math import sqrt

Field = list[list[int]]
Node = tuple[int, int]
Limits = tuple[int, int]


@dataclass(order=True)
class NodeDist:
    dist: int
    item: Node
    pred: Node


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


def parse_input(lines: list[str]) -> Field:
    return [[int(x) for x in line] for line in lines]


def get_neighbors(
    n: Node, x_limits: tuple[int, int], y_limits: tuple[int, int]
) -> list[Node]:
    neighbors = []
    (x, y) = n
    if x_limits[0] <= x - 1:
        neighbors.append((x - 1, y))
    if x + 1 <= x_limits[1]:
        neighbors.append((x + 1, y))

    if y_limits[0] <= y - 1:
        neighbors.append((x, y - 1))
    if y + 1 <= y_limits[1]:
        neighbors.append((x, y + 1))

    return neighbors


def dijkstra(f: Field) -> list[list[NodeDist]]:
    x_limits = (0, len(f[0]) - 1)
    y_limits = (0, len(f) - 1)
    n_nodes = (x_limits[1] + 1) * (y_limits[1] + 1)

    upper_limit = max([max(l) for l in f]) * n_nodes
    nd_f = [
        [NodeDist(upper_limit, (x, y), (x, y)) for x in range(len(f[y]))]
        for y in range(len(f))
    ]

    visited = 0
    current_dist = sqrt(pow(x_limits[1], 2) + pow(y_limits[1], 2))

    # TODO: implement dijkstras algorithm
    nd_f[0][0].dist = 0

    for y in range(len(f)):
        for x in range(len(f[y])):
            visited += 1
            node = nd_f[y][x]
            current_dist = min(
                sqrt(
                    pow(x_limits[1] - 1 - node.item[1], 2)
                    + pow(y_limits[1] - 1 - node.item[1], 2)
                ),
                current_dist,
            )

            print(
                "\t\t\t{:3d}%: {:7d}/{:7d} \t dist: {}".format(
                    (visited * 100) // n_nodes,
                    visited,
                    n_nodes,
                    current_dist,
                ),
                end="\r",
            )

            for _neigh in get_neighbors(node.item, x_limits, y_limits):
                neigh = nd_f[_neigh[1]][_neigh[0]]
                dist = node.dist + f[_neigh[1]][_neigh[0]]
                if dist < neigh.dist:
                    neigh.dist = dist
                    neigh.pred = node.item
    return nd_f


def main() -> None:
    field = parse_input(read_input())
    nd_f = dijkstra(field)
    ly = len(field) - 1
    lx = len(field[ly]) - 1
    final = nd_f[ly][lx]
    print(final.dist)


if __name__ == "__main__":
    main()
