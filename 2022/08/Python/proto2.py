field = list[list[int]]
coord_set = set[tuple[int, int]]


def write_pgm(filename: str, data: field) -> None:
    max_val = max([max(v) for v in data])
    lines = f"P2\n{len(data[0])} {len(data)}\n{max_val}\n"

    pix_data = "\n".join([" ".join([str(v) for v in row]) for row in data])
    lines += pix_data

    with open(filename, "x") as f:
        f.writelines(lines)


def write_ppm(filename: str, data: field, vis: coord_set) -> None:
    def color(x: int, y: int, v: int, m_v: int, vis: set[tuple[int, int]]) -> str:
        return f"{m_v} 0 0" if (x, y) in vis else f"{v} {v} {v}"

    max_val = max([max(v) for v in data])
    lines = f"P3\n{len(data[0])} {len(data)}\n{max_val}\n"

    pix_data = "\n".join([" ".join([color(x, y, v, max_val, vis)
                         for x, v in enumerate(row)]) for y, row in enumerate(data)])
    lines += pix_data

    with open(filename, "x") as f:
        f.writelines(lines)


def calc_score(x: int, y: int, data: field) -> int:
    dim = len(data)
    h = data[y][x]
    blocked = [False, False, False, False]
    v = [0, 0, 0, 0]
    for i in range(1, dim):
        c = x + i
        if not blocked[0] and c < dim:
            if data[y][c] <= h:
                v[0] += 1
                if data[y][c] == h:
                    blocked[0] = True
            else:
                v[0] += 1
                blocked[0] = True

        c = x - i
        if not blocked[1] and c >= 0:
            if data[y][c] <= h:
                v[1] += 1
                if data[y][c] == h:
                    blocked[1] = True
            else:
                v[1] += 1
                blocked[1] = True

        c = y + i
        if not blocked[2] and c < dim:
            if data[c][x] <= h:
                v[2] += 1
                if data[c][x] == h:
                    blocked[2] = True
            else:
                v[2] += 1
                blocked[2] = True

        c = y - i
        if not blocked[3] and c >= 0:
            if data[c][x] <= h:
                v[3] += 1
                if data[c][x] == h:
                    blocked[3] = True
            else:
                v[3] += 1
                blocked[3] = True

    print(v)
    return v[0] * v[1] * v[2] * v[3]


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
visibles: coord_set = set()

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

max_score = max([calc_score(x, y, input_lines) for x, y in visibles])
print(max_score)
