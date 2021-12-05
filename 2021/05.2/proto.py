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

    def get_points(self) -> list[Point]:
        delta_x = self._b.x - self._a.x
        delta_y = self._b.y - self._a.y

        if delta_x == 0:
            return [Point(self._a.x, y) for y in range(min(self._a.y, self._b.y), max(self._a.y, self._b.y) + 1)]
        elif delta_y == 0:
            return [Point(x, self._a.y) for x in range(min(self._a.x, self._b.x), max(self._a.x, self._b.x) + 1)]
        else:
            x_step = delta_x // abs(delta_x)
            y_step = delta_y // abs(delta_y)

            points = []
            for i in range(abs(delta_x) + 1):
                points.append(
                    Point(self._a.x + i * x_step, self._a.y + i * y_step))
            return points


def line_parser(lst: list[str]) -> list[Line]:
    res = []
    for line in lst:
        line_points = line.split(" -> ")
        p_a = line_points[0].split(",")
        p_b = line_points[1].split(",")
        a = Point(int(p_a[0]), int(p_a[1]))
        b = Point(int(p_b[0]), int(p_b[1]))
        l = Line(a, b)
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
