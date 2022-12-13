import matplotlib.pyplot as plt
import matplotlib.ticker as plticker
import numpy as np
import png


def read_path() -> list[(int, int)]:
    path = []
    with open("path.txt", "r") as f:
        for line in f.readlines():
            x, y = line.strip().split(" ")
            path.append((int(x), int(y)))
    return path


def convert(c: chr) -> int:
    return (ord(c) - ord('a'))


def color(v: int) -> tuple[int, int, int]:
    if v >= 0:
        c = min(v * 255//ord('z'), 255)
        return (c, c, c)
    if v == -14:
        return (255, 0, 0)
    if v == -28:
        return (0, 255, 0)


def write_png(data):
    path = read_path()
    height = len(data)
    width = len(data[0])

    img = []
    for y in range(height):
        row = ()
        for x in range(width):
            if (x, y) in path:
                row = row + (0, 0, 255)
            else:
                row = row + color(data[y][x])
        img.append(row)
    with open("map.png", "wb") as f:
        w = png.Writer(width, height, greyscale=False)
        w.write(f, img)


def contour(data):
    height = len(data)
    width = len(data[0])

    path = read_path()
    xs = [x for x, _ in path]
    ys = [y for _, y in path]

    X, Y = np.meshgrid(range(width), range(height))

    locA = plticker.MultipleLocator(base=5)
    locB = plticker.MultipleLocator(base=5)

    # cmap="autumn_r",

    ax = plt.axes()
    ax.xaxis.set_major_locator(locA)
    ax.yaxis.set_major_locator(locB)
    ax.grid(which='major', axis='both', linestyle='-', alpha=0.3)
    ax.contour(X, Y, data, (ord('z')-ord('a'))*2,
               linewidths=0.5, cmap="gnuplot")
    ax.plot(xs, ys, c="black", linewidth=2, linestyle=":")
    ax.axes.set_aspect('equal')
    plt.show()


def main() -> None:
    with open("input.txt", "r") as f:
        data = [[convert(c) for c in line.strip()] for line in f.readlines()]

    # write_png(data)
    contour(data)


if __name__ == "__main__":
    main()
