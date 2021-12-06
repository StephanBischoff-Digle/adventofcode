#!/usr/bin/env python3

import fileinput
from collections import Counter, defaultdict


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


def parse(lst: list[str]) -> dict[int, int]:
    assert len(lst) == 1
    init = [int(x) for x in lst[0].split(",")]
    ret = defaultdict(lambda: 0)
    ret.update(dict(Counter(init)))
    return ret


def main() -> None:
    in_lst = read_input()
    fish = parse(in_lst)

    for _ in range(256):
        tmp_0 = fish[0]
        # regular decrement
        for k in range(8):
            fish[k] = fish[k + 1]

        # 0s added to 6s and add new fish
        fish[6] = fish[6] + tmp_0
        fish[8] = tmp_0

    print(sum(fish.values()))


if __name__ == "__main__":
    main()
