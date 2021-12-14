#!/usr/bin/env python3

import fileinput
from collections import defaultdict

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


def dfs(
    v: str, graph: Graph, visited: set[str], allow_revisit: bool, cp: str
) -> set[str]:
    if v == "end":
        return set([cp])

    paths = set()
    if v.islower():
        visited.add(v)
    for node in graph[v]:
        if node not in visited:
            paths = paths.union(
                dfs(node, graph, visited, allow_revisit, f"{cp},{node}")
            )

    if v in visited and v != "start":
        visited.remove(v)

        # This generates some duplicates of which I don't yet know how
        # to get rid of elegantly, so I just dump all the path-strings
        # into a set to get rid of them that way :(
        if allow_revisit:
            for node in graph[v]:
                if node not in visited:
                    paths = paths.union(
                        dfs(node, graph, visited, False, f"{cp},{node}")
                    )

    return paths


def paths(graph: Graph) -> int:
    visited = set(["start"])
    n_paths = dfs("start", graph, visited, True, "start")
    return len(n_paths)


def main() -> None:
    in_lst = read_input()
    graph = parse_graph(in_lst)
    print(paths(graph))


if __name__ == "__main__":
    main()
