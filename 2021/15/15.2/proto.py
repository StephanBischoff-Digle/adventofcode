#!/usr/bin/env python3

import fileinput
from dataclasses import dataclass
from collections import defaultdict
from math import sqrt
from queue import PriorityQueue

Field = list[list[int]]
Node = tuple[int, int]
Limits = tuple[int, int]


@dataclass(order=True)
class NodeDist:
    priority: int
    item: Node
    pred: Node


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


def parse_input(lines: list[str]) -> Field:
    return [[int(x) for x in line] for line in lines]


def get_neighbors(n: Node, x_limits: int, y_limits: int) -> list[Node]:
    neighbors = []
    (x, y) = n
    if 0 <= x - 1:
        neighbors.append((x - 1, y))
    if x + 1 <= 5 * x_limits - 1:
        neighbors.append((x + 1, y))

    if 0 <= y - 1:
        neighbors.append((x, y - 1))
    if y + 1 <= 5 * y_limits - 1:
        neighbors.append((x, y + 1))

    return neighbors


def risk_level(n: Node, f: Field) -> int:
    x_limits = len(f[0])
    y_limits = len(f)
    (x, y) = n
    mapped_x = x % x_limits
    mapped_y = y % y_limits
    modifier = x // x_limits + y // y_limits

    risk = 1 + ((f[mapped_y][mapped_x]) + modifier - 1) % 9
    return risk


def dijkstra(f: Field) -> dict[Node, NodeDist]:
    x_limits = len(f[0])
    y_limits = len(f)
    n_nodes = (x_limits) * (y_limits) * 25

    ly = len(f) * 5 - 1
    lx = len(f[len(f) - 1]) * 5 - 1
    target_node = (lx, ly)

    upper_limit = 9 * n_nodes
    print(upper_limit)
    nd_f = defaultdict(lambda: NodeDist(upper_limit, (0, 0), (-1, -1)))

    visited = set()
    nd_f[(0, 0)].priority = 0
    border = PriorityQueue()
    border.put(nd_f[(0, 0)])

    while not border.empty():
        print(
            "\t\t{:3d}%\t{:10d}/{:10d}".format(
                (100 * len(visited)) // n_nodes, len(visited), n_nodes
            ),
            end="\r",
        )
        node = border.get()
        if node.item == target_node:
            break
        if node.item in visited:
            continue

        visited.add(node.item)
        for ns in get_neighbors(node.item, x_limits, y_limits):
            neigh = nd_f[(ns[0], ns[1])]
            neigh.item = (ns[0], ns[1])

            if ns not in visited:
                dist = node.priority + risk_level(ns, f)
                if dist < neigh.priority:
                    neigh.priority = dist
                    neigh.pred = node.item
            border.put(neigh)

    return nd_f


def main() -> None:
    field = parse_input(read_input())
    nd_f = dijkstra(field)
    ly = len(field) * 5 - 1
    lx = len(field[len(field) - 1]) * 5 - 1
    final = nd_f[(lx, ly)]
    print()
    print(final.priority)


if __name__ == "__main__":
    main()
