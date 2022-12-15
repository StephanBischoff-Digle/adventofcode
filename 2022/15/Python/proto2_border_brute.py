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


def border_points(sensor: Point, r: int, hi: int) -> list[Point]:
    sx, sy = sensor
    points = []
    r = r + 1
    for y in range(max(0, sy-r), min(hi, sy+r)+1):
        xr = r - abs(sy - y)
        if sx-xr >= 0:
            points.append((sx-xr, y))
        if sx+xr <= hi:
            points.append((sx+xr, y))
    return points


def main() -> None:
    sensors = read_input("input.txt")
    # sensors = read_input("test.txt")
    hi = 4000000

    b_points = set()
    for idx, (sensor, _, r) in enumerate(sensors):
        print_progress("collecting border points", 40, idx, len(sensors))
        # print(f"Sensor {idx} of {len(sensors)}")
        for p in border_points(sensor, r, hi):
            b_points.add(p)

    print_progress("collecting border points", 40, 1, 1)
    print()

    # check border_points if they are in coverage
    lx, ly = (0, 0)
    for idx, (px, py) in enumerate(b_points):
        if idx % 3000 == 0:
            print_progress("checking points         ", 40,
                           idx//3000, len(b_points)//3000)
        in_cover = False
        for (sx, sy), _, r in sensors:
            if dist(sx, sy, px, py) <= r:
                in_cover = True
                break
        if not in_cover:
            lx = px
            ly = py
            break

    print(f"\n{lx=} {ly=} freq: {lx*hi + ly}")
    exit(0)


if __name__ == "__main__":
    main()
