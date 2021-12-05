#!/usr/bin/env python3

import fileinput


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


class Point(object):
    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y

    def __hash__(self) -> int:
        return hash((self.x, self.y))

    def __eq__(self, other: "Point") -> bool:
        return self.x == other.x and self.y == other.y


class Line(object):
    def __init__(self, a: Point, b: Point) -> None:
        self._a = a
        self._b = b

    def is_diagonal(self) -> bool:
        delta_x = abs(self._a.x - self._b.x)
        delta_y = abs(self._a.y - self._b.y)
        return delta_x != 0 and delta_y != 0

    def get_points(self) -> list[Point]:
        # Due to the diagonality check I can assume, that either dx or dy is 0
        # Which I'll use as the axis to iterate over.
        delta_x = abs(self._a.x - self._b.x)
        delta_y = abs(self._a.y - self._b.y)
        if delta_x == 0:
            return [Point(self._a.x, y) for y in range(min(self._a.y, self._b.y), max(self._a.y, self._b.y) + 1)]
        if delta_y == 0:
            return [Point(x, self._a.y) for x in range(min(self._a.x, self._b.x), max(self._a.x, self._b.x) + 1)]


def line_parser(lst: list[str]) -> list[Line]:
    res = []
    for line in lst:
        line_points = line.split(" -> ")
        p_a = line_points[0].split(",")
        p_b = line_points[1].split(",")
        a = Point(int(p_a[0]), int(p_a[1]))
        b = Point(int(p_b[0]), int(p_b[1]))
        l = Line(a, b)
        if not l.is_diagonal():
            res.append(l)

    return res


def main() -> None:
    in_lst = read_input()
    segments = line_parser(in_lst)
    occupied = {}
    for segment in segments:
        for point in segment.get_points():
            occupied[point] = occupied.get(point, 0) + 1

    solution = len(list(filter(lambda v: v >= 2, occupied.values())))
    print(solution)


if __name__ == "__main__":
    main()
