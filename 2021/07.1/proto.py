#!/usr/bin/env python3

import fileinput


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


def find_min(lst: list[int]) -> int:
    start = min(lst)
    prev = sum(lst)
    current = prev

    while prev >= current:
        start += 1
        prev = current
        current = sum([abs(start - l) for l in lst])

    return prev


in_lst = [int(x) for x in read_input()[0].split(",")]
print(find_min(in_lst))
