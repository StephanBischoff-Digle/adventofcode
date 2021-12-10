#!/usr/bin/env python3


import fileinput
from collections import deque


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


scores = {")": 3, "]": 57, "}": 1197, ">": 25137}
close = {")": "(", "]": "[", "}": "{", ">": "<"}


def main() -> None:
    in_lst = read_input()

    vals = []
    for line in in_lst:
        stack = deque()

        val = 0
        for c in line:
            if c in close.values():
                stack.append(c)
            else:
                o = stack.pop()
                if o != close[c]:
                    val = scores[c]
                    vals.append(val)
                    break
    print(sum(vals))


if __name__ == "__main__":
    main()
