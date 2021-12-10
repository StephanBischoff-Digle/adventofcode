#!/usr/bin/env python3


import fileinput
from collections import deque


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


scores = {"(": 1, "[": 2, "{": 3, "<": 4}
close = {")": "(", "]": "[", "}": "{", ">": "<"}


def main() -> None:
    in_lst = read_input()

    vals = []
    for line in in_lst:
        stack = deque()

        broken = False
        for c in line:
            if c in close.values():
                stack.append(c)
            else:
                o = stack.pop()
                if o != close[c]:
                    broken = True
                    break
        if not broken:
            val = 0
            while stack:
                val = val * 5 + scores[stack.pop()]
            vals.append(val)
    print(sorted(vals)[len(vals) // 2])


if __name__ == "__main__":
    main()
