#!/usr/bin/env python3

import fileinput

measures = [int(line.strip()) for line in fileinput.input("input.txt")]
summs = [sum(measures[i : i + 3]) for i in range(len(measures) - 2)]

solution = [summs[i] < summs[i + 1] for i in range(len(summs) - 1)].count(True)
print(solution)
