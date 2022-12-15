from typing import Optional

Point = tuple[int, int]


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
    # merge intervals
    intervals = list(
        map(lambda x: [max(0, x[0]), min(hi, x[1])],
            filter(lambda x: x[0] <= hi and x[1] >= 0, intervals)))
    intervals.sort()
    # print(f"{intervals=}")
    stack = []
    stack.append(intervals[0])
    for i in intervals[1:]:
        # print(f"{stack=}")
        if stack[-1][0]-1 <= i[0] and i[0] <= stack[-1][1]+1:
            stack[-1][1] = max(stack[-1][1], i[1])
        else:
            stack.append(i)

    if len(stack) > 1:
        return stack[0][1]+1
    return None


def main() -> None:
    sensors = read_input("input.txt")
    # sensors = read_input("test.txt")
    # max_x = 20
    max_x = 4000000
    for y in range(max_x+1):
        if y % 1000 == 0:
            print(f"{y:7} -> {(y*100)//max_x:5}%")
        intervals = []
        for point, _, r in sensors:
            intervals.append(coverage_of_at(point, r, y))
        x = find_missing(intervals, max_x)
        if x is not None:
            print(f"{x=} {y=}")
            print(f"{x*4000000 + y}")
            break


if __name__ == "__main__":
    main()
