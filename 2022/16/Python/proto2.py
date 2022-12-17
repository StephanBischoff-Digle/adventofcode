from itertools import combinations

Matrix = list[list[int]]


def parse_valve(in_str: str):
    name = in_str[6:8]
    semi_split = in_str.split("=")[1].split(";")
    rate = int(semi_split[0])
    tunnel = [t.strip() for t in semi_split[1].removeprefix(
        " tunnel leads to valve ").removeprefix(
        " tunnels lead to valves ").split(",")]

    return {"name": name, "rate": rate, "tunnel": tunnel}


def generate_graphviz(graph) -> None:
    out = "graph Tunnels {\n"
    out += "  esep=5;\n"
    out += "  esep=5;\n  overlap=scale;\n  sep = .1;\n"

    edges = set()
    for k, v in graph.items():
        rate = v["rate"]
        out += f"  {k} [label=\"{k}: {rate}\"];\n"
        for c in v["tunnel"]:
            edges.add((min(c, k), max(c, k)))

    for a, b in edges:
        out += f"  {a} -- {b} [label=\"1\"];\n"
    out += "}\n"
    print(out)


def calc_path_lengths(graph: dict) -> dict:
    """Creates the weighted adjacency matrix of the graph"""
    lengths = {}
    for k in sorted(graph.keys()):
        queue = []
        visited = {}
        d = 0
        queue.append((k, d))
        visited[k] = d
        while len(queue) > 0:
            (v, d) = queue.pop(0)
            for u in graph[v]["tunnel"]:
                if u not in visited.keys():
                    queue.append((u, d+1))
                    visited[u] = d + 1
        visited = {k: v + (1 if v > 0 else 0)
                   for k, v in sorted(visited.items())}
        lengths[k] = visited
    return lengths


def compute_flow(path: list[int], rates: list[int], m: Matrix, hi: int = 30) -> int:
    """computes the flow of the path"""
    t = 0
    cf = 0
    cn = 0
    total = 0
    for i in path:
        dt = m[cn][i]
        cn = i
        total += dt * cf
        cf += rates[i]
        t += dt
    total += (hi - t) * cf
    return total


def find_max_flow(paths, rates: list[int], m: Matrix, hi: int = 30) -> int:
    """
    from the paths given, find the one with the maximum flow and return it
    """
    max_flow = 0
    for path in paths:
        flow = compute_flow(path[:-1], rates, m, hi)
        max_flow = max(max_flow, flow)
    return max_flow


def permutations(current: int, allowed: list[int], rem_time: int, m: Matrix) -> list[int]:
    """
    generates a index permutation on demand.
    the paths generated are sorted by maximum coverage first, taking the least
    amount of time between the nodes in the `allowed` index buffer.
    this generator ensures that the given `rem_time` is not exceeded.
    """
    s = sorted([(idx, v)
                for idx, v in enumerate(m[current]) if v > 0 and idx in allowed], key=lambda x: x[1])

    if len(s) == 0:
        yield [-1]
    else:
        for i, v in s:
            if rem_time - v > 0:
                n_allowed = [j for j in allowed if j != i]
                for perm in permutations(i, n_allowed, rem_time - v, m):
                    yield [i] + perm
            else:
                yield [-1]


def main() -> None:
    valves = {}

    with open("input.txt", "r") as f:
        for line in f.readlines():
            valve = parse_valve(line)
            valves[valve["name"]] = {
                "rate": valve["rate"], "tunnel": valve["tunnel"]}
    lengths = calc_path_lengths(valves)
    m = []
    rates = []

    # remove 0 value nodes from the adjacency matrix
    for k, v in lengths.items():
        if k == "AA" or valves[k]["rate"] > 0:
            m.append([a for b, a in lengths[k].items()
                     if valves[b]["rate"] > 0 or b == "AA"])
            rates.append(valves[k]["rate"])

    postfix = "                              "
    time = 26

    indizes = list(range(1, len(rates)))
    print(len(indizes))
    m_flow = 0
    k = 8
    print("========================================")
    for j, elephants in enumerate(combinations(indizes, k)):
        print(f"\r{j=:10}{postfix}", end="")
        my = [idx for idx in indizes if idx not in elephants]
        ele_flow = find_max_flow(permutations(
            0, list(elephants), time, m), rates, m, time)
        my_flow = find_max_flow(permutations(
            0, my, time, m), rates, m, time)
        flow = my_flow + ele_flow
        if m_flow < flow:
            print(f"\r{j=:10} {flow=}")
            m_flow = flow

    print()
    print("========================================")
    print(m_flow)
    print("========================================")


if __name__ == "__main__":
    main()
