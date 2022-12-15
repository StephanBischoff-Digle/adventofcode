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


# https://www.reddit.com/r/adventofcode/comments/zmcn64/comment/j0b90nr
def main() -> None:
    sensors = read_input("input.txt")

    a_coeffs = set()
    b_coeffs = set()
    for (x, y), _, r in sensors:
        a_coeffs.add(y-x+r+1)
        a_coeffs.add(y-x-r-1)
        b_coeffs.add(x+y+r+1)
        b_coeffs.add(x+y-r-1)

    bound = 4000000
    for a in a_coeffs:
        for b in b_coeffs:
            px = (b-a)//2
            py = (a+b)//2
            if 0 <= px <= bound and 0 <= py <= bound:
                if all(dist(sensor[0][0], sensor[0][1], px, py) > sensor[2] for sensor in sensors):
                    print(f"{px=} {py=} freq: {px*bound+py}")
                    return


if __name__ == "__main__":
    main()
