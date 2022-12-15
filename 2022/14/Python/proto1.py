import png

Map = list[list[int]]
CLine = list[tuple[int, int]]


def simulation_step(m: Map, in_x: int) -> tuple[Map, bool]:
    x = in_x
    y = 0

    while True:
        # Simulation is done
        if y == len(m)-1:
            return (m, True)

        if m[y+1][x] == 0:
            y += 1
            continue
        if m[y+1][x-1] == 0:
            y += 1
            x -= 1
            continue
        if m[y+1][x+1] == 0:
            y += 1
            x += 1
            continue
        break
    m[y][x] = 2

    return (m, False)


def clamp(v, lo, hi):
    return min(hi, max(lo, v))


def read_data() -> list[CLine]:
    with open("input.txt", "r") as f:
        m_lines = []
        for line in f.readlines():
            coords = []
            for coord in line.strip().split("->"):
                x, y = coord.split(",")
                coords.append((int(x), int(y)))
            m_lines.append(coords)
    return m_lines


def print_map(m: Map):
    t = ['.', '#', '\033[32;1mo\033[0m']
    for y in m:
        print(''.join(list(map(lambda v: t[v], y))))


def map_png(m: Map, filename: str):
    height = len(m)
    width = len(m[0])

    cmap = [(0, 0, 0), (255, 255, 255), (194, 178, 128)]

    img = []
    for y in range(height):
        row = ()
        for x in range(width):
            row = row + cmap[m[y][x]]
        img.append(row)
    with open(filename, "wb") as f:
        w = png.Writer(width, height, greyscale=False)
        w.write(f, img)


def bounding_box(lines: list[CLine]) -> tuple[tuple[int, int], int]:
    xs = []
    ys = []
    for line in lines:
        xs.extend([x for x, _ in line])
        ys.extend([y for _, y in line])

    min_x = min(xs)-1
    max_x = max(xs)+2

    max_y = max(ys)+1
    return ((min_x, max_x), max_y)


def generate_map(lines: list[CLine]):
    (min_x, max_x), max_y = bounding_box(lines)
    delta_x = max_x - min_x

    m = [[0]*delta_x for _ in range(max_y)]

    for line in lines:
        for idx in range(1, len(line)):
            start_x = line[idx-1][0]-min_x
            diff_x = line[idx][0] - line[idx-1][0]
            delta_x = clamp(diff_x, -1, 1)

            start_y = line[idx-1][1]
            diff_y = line[idx][1] - line[idx-1][1]
            delta_y = clamp(diff_y, -1, 1)

            for i in range(max(abs(diff_x), abs(diff_y))+1):
                m[i*delta_y + start_y][i*delta_x + start_x] = 1
    return m


def count_sand(m: Map) -> int:
    count = 0
    for row in m:
        count += len(list(filter(lambda x: x == 2, row)))
    return count


if __name__ == "__main__":
    lines = read_data()
    m = generate_map(lines)
    (min_x, _), _ = bounding_box(lines)
    injection = 500 - min_x

    done = False
    while not done:
        m, done = simulation_step(m, injection)
    print(count_sand(m))
    # map_png(m, "filled_1.png")
    print_map(m)
