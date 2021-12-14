#!/usr/bin/env python3

import fileinput
from collections import defaultdict


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


def main() -> None:
    in_lst = read_input()
    lst = [line.split(" | ")[1] for line in in_lst]

    easy_map = defaultdict(lambda: 0)
    easy_map.update({
        2: 1,
        3: 1,
        4: 1,
        7: 1,
    })

    solution = sum([sum(map(lambda x: easy_map[len(x)], [
                   w for w in line.split(" ")])) for line in lst])

    print(solution)


if __name__ == "__main__":
    main()
