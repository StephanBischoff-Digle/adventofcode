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


def coverage_of_at(point: Point, r: int, row: int) -> list[int]:
    px, py = point
    r = r - abs(row-py)
    return [px+x-r for x in range(2*r+1)]


def main() -> None:
    sensors = read_input("input.txt")
    # sensors = read_input("test.txt")
    target_row = 2000000
    coverage = set()
    for point, _, r in sensors:
        cov = coverage_of_at(point, r, target_row)
        for x in cov:
            coverage.add(x)
    beacons = [beacon for _, beacon, _ in sensors if beacon[1] == target_row]

    for beacon in beacons:
        coverage.discard(beacon[0])

    # print(coverage)
    print(len(coverage))


if __name__ == "__main__":
    main()
