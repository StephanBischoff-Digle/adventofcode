def write_pgm(filename: str, data: list[list[int]]) -> None:
    max_val = max([max(v) for v in data])
    lines = f"P2\n{len(data[0])} {len(data)}\n{max_val}\n"

    pix_data = "\n".join([" ".join([str(v) for v in row]) for row in data])
    lines += pix_data

    with open(filename, "x") as f:
        f.writelines(lines)


def write_ppm(filename: str, data: list[list[int]], vis: set[tuple[int, int]]) -> None:
    def color(x: int, y: int, v: int, m_v: int, vis: set[tuple[int, int]]) -> str:
        return f"{m_v} 0 0" if (x, y) in vis else f"{v} {v} {v}"

    max_val = max([max(v) for v in data])
    lines = f"P3\n{len(data[0])} {len(data)}\n{max_val}\n"

    pix_data = "\n".join([" ".join([color(x, y, v, max_val, vis)
                         for x, v in enumerate(row)]) for y, row in enumerate(data)])
    lines += pix_data

    with open(filename, "x") as f:
        f.writelines(lines)


with open("input.txt", "r") as f:
    input_lines = [[int(v) for v in line.strip()] for line in f.readlines()]

    # write_pgm("forest.pgm", input_lines)


# forest is square!
assert len(input_lines) == len(input_lines[0])
dim = len(input_lines)

# pre-process values after reading.
# I want to store the tree coords in a set, so I store them together with the
# hight inside a triple.
coorded_trees = [[(x, y, h) for (x, h) in enumerate(row)]
                 for (y, row) in enumerate(input_lines)]

# set of visible tree coordinates
visibles: set[tuple[int, int]] = set()

# hight kernel
kernel = {
    "l": [-1] * dim,
    "r": [-1] * dim,
    "t": [-1] * dim,
    "b": [-1] * dim,
}

for i in range(dim):
    j = (dim-1) - i

    for p in range(dim):
        # top layer
        x, y, h = coorded_trees[i][p]
        if h > kernel["t"][p]:
            visibles.add((x, y))
            kernel["t"][p] = h

        # bottom layer
        x, y, h = coorded_trees[j][p]
        if h > kernel["b"][p]:
            visibles.add((x, y))
            kernel["b"][p] = h

        # left layer
        x, y, h = coorded_trees[p][i]
        if h > kernel["l"][p]:
            visibles.add((x, y))
            kernel["l"][p] = h

        # right layer
        x, y, h = coorded_trees[p][j]
        if h > kernel["r"][p]:
            visibles.add((x, y))
            kernel["r"][p] = h

# write_ppm("vis.ppm", input_lines, visibles)
print(len(visibles))
