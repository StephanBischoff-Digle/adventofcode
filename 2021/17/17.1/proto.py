#!/usr/bin/env python3

import fileinput
import logging as log

log.basicConfig(format="\033[1;34m%(levelname)s\033[0m:%(message)s", level=log.DEBUG)


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


def parse_input(in_str: str) -> tuple[tuple[int, int], tuple[int, int]]:
    s0 = in_str.split(": x=")[1]
    xy = s0.split(", y=")
    sx = xy[0].split("..")
    xs = (int(sx[0]), int(sx[1]))

    sy = xy[1].split("..")
    ys = (int(sy[0]), int(sy[1]))

    return (xs, ys)


def main() -> None:
    in_lst = read_input()
    area = parse_input(in_lst[0])
    log.debug(f"Area: {area}")
    pos = min(area[1])

    log.debug(f"Max Y Speed: {abs(pos)}")

    b = sum(range(abs(pos)))
    log.info(f"\033[1;31mSolution: {b}\033[0m")


if __name__ == "__main__":
    main()
