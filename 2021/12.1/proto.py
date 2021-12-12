#!/usr/bin/env python3

import fileinput
from collections import defaultdict, deque
import json

Graph = dict[str, list[str]]


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


def parse_graph(edges: list[str]) -> Graph:
    graph = defaultdict(lambda: [])
    for edge in edges:
        nodes = edge.split("-")
        graph[nodes[0]].append(nodes[1])
        graph[nodes[1]].append(nodes[0])
    return graph


def dfs(v: str, graph: Graph, visited: set[str]) -> int:
    if v == "end":
        return 1

    paths = 0
    if v.islower():
        visited.add(v)
    for node in graph[v]:
        if node not in visited:
            paths += dfs(node, graph, visited)

    if v in visited:
        visited.remove(v)
    return paths


def paths(graph: Graph) -> int:
    visited = set()
    n_paths = dfs("start", graph, visited)
    return n_paths


def main() -> None:
    in_lst = read_input()
    graph = parse_graph(in_lst)
    print(paths(graph))


if __name__ == "__main__":
    main()
