from typing import Optional

Point = tuple[int, int]


def print_progress(prefix: str, width: int, i: int, hi: int) -> None:
    n = int((width * i) // hi)
    k = width - n
    n_s = "#"*n
    k_s = " "*k

    print(f"\r{prefix}: [{n_s}{k_s}]  {i * 100/hi:5.1f}%", end="")


def dist(xa: int, ya: int, xb: int, yb: int) -> int:
    return abs(xa-xb)+abs(ya-yb)


def read_input(filename: str) -> list[tuple[Point, Point, int]]:
    lst = []
    with open(filename, "r") as f:
        lines = f.readlines()
        for line in lines:
            line = line.strip()
            sensor, beacon = line.split(":")
            sensor_x, sensor_y = sensor.split(",")
            sx = int(sensor_x.split("=")[1])
            sy = int(sensor_y.strip().split("=")[1])

            beacon_x, beacon_y = beacon.split(",")
            bx = int(beacon_x.split("=")[1])
            by = int(beacon_y.strip().split("=")[1])

            lst.append(((sx, sy), (bx, by), dist(sx, sy, bx, by)))
    return lst


def coverage_of_at(point: Point, r: int, row: int) -> list[int, int]:
    px, py = point
    r = r - abs(row-py)
    if r <= 0:
        return [-1, -1]
    return [px-r, px+r]


def find_missing(intervals: list[list[int, int]], hi=20) -> Optional[int]:
    # filter intervals that are out of bounds or clmap them to the bounds
    intervals = list(
        map(lambda x: [max(0, x[0]), min(hi, x[1])],
            filter(lambda x: x[0] <= hi and x[1] >= 0, intervals)))

    # start with the lowest min
    intervals.sort()

    stack = []
    stack.append(intervals[0])
    for i in intervals[1:]:
        # check if the intervals are overlapping or adjecent
        if stack[-1][0]-1 <= i[0] and i[0] <= stack[-1][1]+1:
            # update the max val of the biggest intervall
            stack[-1][1] = max(stack[-1][1], i[1])
        else:
            # add a new disjunkt interval
            stack.append(i)

    # check if more than one interval was found
    # if so, take the smallest one's max and return the succesor
    if len(stack) > 1:
        return stack[0][1]+1

    # all intervals overlapping, report None missing
    return None


def main() -> None:
    sensors = read_input("input.txt")
    # sensors = read_input("test.txt")
    # max_x = 20
    max_x = 4000000
    for y in range(max_x+1):
        if y % 100 == 0:
            print_progress("merging interval", 50, y / 100, (max_x+1)/100)
        intervals = []
        for point, _, r in sensors:
            intervals.append(coverage_of_at(point, r, y))
        x = find_missing(intervals, max_x)
        if x is not None:
            print()
            print(f"{x=} {y=}")
            print(f"{x*4000000 + y}")
            break


if __name__ == "__main__":
    main()
