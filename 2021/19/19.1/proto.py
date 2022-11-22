#!/usr/bin/env python3

import fileinput
import logging as log
from itertools import combinations, permutations
from collections import defaultdict, deque
from typing import Union, Optional
from math import sin, cos, tau


log.basicConfig(
    format="\033[1;34m%(levelname)s\033[0m: %(message)s", level=log.DEBUG)


class Point3D(object):
    @staticmethod
    def from_str(line: str) -> "Point3D":
        [x, y, z] = [int(v) for v in line.split(",")]
        return Point3D(x, y, z)

    def __init__(self, x: int, y: int, z: int) -> None:
        self.x = x
        self.y = y
        self.z = z

    def mag(self) -> float:
        return pow(self.x, 2) + pow(self.y, 2) + pow(self.z, 2)

    def __add__(self, other) -> "Point3D":
        x = self.x + other.x
        y = self.y + other.y
        z = self.z + other.z
        return Point3D(x, y, z)

    def __sub__(self, other) -> "Point3D":
        x = self.x - other.x
        y = self.y - other.y
        z = self.z - other.z
        return Point3D(x, y, z)

    def __eq__(self, other) -> bool:
        return self.x == other.x and self.y == other.y and self.z == other.z

    def __repr__(self) -> str:
        return f"({self.x}, {self.y}, {self.z})"

    def __hash__(self) -> int:
        return hash((self.x, self.y, self.z))


class Scanner(object):
    @staticmethod
    def from_str(lines: list[str]) -> "Scanner":
        name = lines[0][4:-4]
        readings = [Point3D.from_str(line) for line in lines[1:]]
        return Scanner(name, readings)

    def __init__(self, name: str, readings: list[Point3D]) -> None:
        self.name = name
        self.readings = readings
        self._distances = []

    def constelations(self) -> list[list[tuple[int, int, float]]]:
        if self._distances:
            return self._distances
        idxs = range(len(self.readings))
        dist_dict = {k: [] for k in idxs}
        for (a, b) in list(combinations(idxs, 2)):
            mag = (self.readings[a] - self.readings[b]).mag()
            dist_dict[a].append((a, b, mag))
            dist_dict[b].append((a, b, mag))

        dists = []
        for i in idxs:
            c_dists = dist_dict[i]
            c_dists.sort()
            dists.append(c_dists)
        self._distances = dists
        return dists

    def __repr__(self) -> str:
        return f"{self.name}"


class Transform(object):
    def __init__(self) -> None:
        self.M: list[list[float]] = [
            [1, 0, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 1]]

    def translate(self, p: Point3D) -> None:
        self.M[0][-1] = p.x
        self.M[1][-1] = p.y
        self.M[2][-1] = p.z

    def rotate(self, a: float = 0, b: float = 0, c: float = 0) -> None:
        Rx = Transform()
        Ry = Transform()
        Rz = Transform()

        c_a = cos(a)
        s_a = sin(a)
        Rx.M[1][1] = c_a
        Rx.M[1][2] = -s_a
        Rx.M[2][1] = s_a
        Rx.M[2][2] = c_a

        c_b = cos(b)
        s_b = sin(b)
        Ry.M[0][0] = c_b
        Ry.M[0][2] = s_b
        Ry.M[2][0] = -s_b
        Ry.M[2][2] = c_b

        c_c = cos(c)
        s_c = sin(c)
        Rz.M[0][0] = c_c
        Rz.M[0][1] = -s_c
        Rz.M[1][0] = s_c
        Rz.M[1][1] = c_c

        R = Transform()
        R = Rz * Ry * Rx
        self = self * R

    @staticmethod
    def find_between(ab: tuple[Point3D, Point3D], xy: tuple[Point3D, Point3D]) -> Optional["Transform"]:
        v_ab = ab[1] - ab[0]
        v_xy = xy[1] - xy[0]

        trans = xy[0] - ab[0]
        valid_rotations = [0, 1/4 * tau, 2/4 * tau, 3/4 * tau]

        for (a, b, c) in permutations(valid_rotations, 3):
            R = Transform()
            R.translate(trans)
            R.rotate(a, b, c)
            project = R * v_xy
            if project == v_xy:
                return R
        return None

    def __mul__(self, other: Union["Transform", Point3D]) -> Union["Transform", Point3D]:
        if isinstance(other, Point3D):
            v = [other.x, other.y, other.z]
            x = int(sum([a * b for a, b in zip(self.M[0], v)]))
            y = int(sum([a * b for a, b in zip(self.M[1], v)]))
            z = int(sum([a * b for a, b in zip(self.M[2], v)]))
            return Point3D(x, y, z)
        else:
            T = Transform()
            for m, row in enumerate(self.M):
                for n, col in enumerate(row):
                    T.M[n][m] = sum([col*other.M[k][n]
                                     for k in range(len(other.M))])
            return T


class Grid(object):
    def __init__(self, scanners: set[Scanner]) -> None:
        self.points: set[Point3D] = set()
        self.transformations: list[Transform] = {}
        self.scanners = {s.name: s for s in scanners}

    def merge(self) -> None:
        scanner_set = deque(self.scanners.values())
        base = scanner_set.pop()
        self.transformations[base.name] = Transform()

        print(f"Base: {base.name}")

        while scanner_set:
            print(f"Queue: {scanner_set}")
            current = scanner_set.pop()
            print(f"{current.name}: {self.transformations.keys()}")
            for k in self.transformations.keys():
                T = try_map_into(self.scanners[k], current)
                print(f"Transformation: {T}")
                if T:
                    print("found transform")
                    self.transformations[current.name] = self.transformations[k] * T
                    break
            if not current.name in self.transformations:
                print("reappend")
                scanner_set.appendleft(current)

        for scanner in self.scanners.values():
            for p in scanner.readings:
                self.points.add(self.transformations[scanner.name] * p)


def try_map_into(a: Scanner, b: Scanner) -> Optional[Transform]:
    print(f"{a.name} -> {b.name}")
    constelation_candidates = defaultdict(lambda: defaultdict(lambda: 0))
    constelations = []
    for pa in a.constelations():
        for (aa, ab, ad) in pa:
            for pb in b.constelations():
                for (ba, bb, bd) in pb:
                    if ad == bd:
                        constelation_candidates[aa][ba] += 1
                        constelation_candidates[aa][bb] += 1
                        constelation_candidates[ab][ba] += 1
                        constelation_candidates[ab][bb] += 1
                        constelations.append(((aa, ab), (ba, bb)))

    if len(constelation_candidates.keys()) >= 12:
        (ab_const, xy_const) = constelations[0]
        ab = (a.readings[ab_const[0]], a.readings[ab_const[1]])
        xy = (b.readings[xy_const[0]], b.readings[xy_const[1]])

        T = Transform.find_between(ab, xy)
        if T:
            for (ab_const, xy_const) in constelations[1:]:
                ab = (a.readings[ab_const[0]], a.readings[ab_const[1]])
                xy = (b.readings[xy_const[0]], b.readings[xy_const[1]])

                px = T * xy[0]
                py = T * xy[1]
                if ab != (px, py):
                    return None

        return T
    return None


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


def main() -> None:
    in_lst = read_input()
    in_lst.append("")

    scanners = set()
    acc = []
    for line in in_lst:
        if line == "":
            scanners.add(Scanner.from_str(acc))
            acc = []
        else:
            acc.append(line)

    G = Grid(scanners)
    G.merge()

    print()
    for p in G.points:
        print(str(p))


if __name__ == "__main__":
    main()
