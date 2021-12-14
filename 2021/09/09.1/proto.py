#!/usr/bin/env python3

import fileinput
import png


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


def check_bounds(
    p: tuple[int, int], bounds: tuple[tuple[int, int], tuple[int, int]]
) -> bool:
    ((x_min, x_max), (y_min, y_max)) = bounds
    (x, y) = p
    return x_min <= x <= x_max and y_min <= y <= y_max


def kernel(pos: tuple[int, int], heights: list[list[int]]) -> list[int]:
    (x, y) = pos
    bounds = ((0, len(heights[y]) - 1), (0, len(heights) - 1))
    adj = list(
        filter(
            lambda p: check_bounds(p, bounds),
            [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)],
        )
    )

    hs = [heights[y][x] for (x, y) in adj]
    return hs


def draw_map(heights: list[list[int]]) -> None:
    img = []

    for y in range(len(heights)):
        row = ()
        for x in range(len(heights[y])):
            val = heights[y][x]
            c_val = 255 // 10 * val
            row = row + (c_val, c_val, c_val)
        img.append(row)

    with open("map.png", "wb") as f:
        w = png.Writer(len(heights[0]), len(heights), greyscale=False)
        w.write(f, img)


def draw_outlines(heights: list[list[int]]) -> None:
    img = []

    for y in range(len(heights)):
        row = ()
        for x in range(len(heights[y])):
            val = heights[y][x]
            c_val = 255 if val == 9 else 0
            row = row + (c_val, c_val, c_val)
        img.append(row)

    with open("map_outline.png", "wb") as f:
        w = png.Writer(len(heights[0]), len(heights), greyscale=False)
        w.write(f, img)


def main() -> None:
    in_lst = read_input()
    heights = [[int(h) for h in line] for line in in_lst]

    mins = []
    for y in range(len(heights)):
        for x in range(len(heights[y])):
            v = heights[y][x]
            if all(map(lambda a: a > v, kernel((x, y), heights))):
                mins.append(v + 1)
    print(sum(mins))
    draw_map(heights)
    draw_outlines(heights)


if __name__ == "__main__":
    main()
