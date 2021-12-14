#!/usr/bin/env python3

import fileinput

with fileinput.input("input.txt") as file:
    acc = {}
    lines = [line.strip() for line in file]

    # count bits
    for line in lines:
        for (idx, c) in enumerate(line):
            acc[idx] = acc.get(idx, 0) + int(c)

    l = len(lines)
    gamma = 0
    mask = 0
    for (k, v) in acc.items():
        # decide if the bit is relevant
        d = 0 if v / l < 0.5 else 1
        gamma += d << (len(acc) - k - 1)
        mask += 1 << (len(acc) - k - 1)

    epsilon = gamma ^ mask
    print("gamma: {}".format(gamma, gamma))
    print("epsilon: {}".format(epsilon, epsilon))

    print(f"solution: {epsilon * gamma}")
