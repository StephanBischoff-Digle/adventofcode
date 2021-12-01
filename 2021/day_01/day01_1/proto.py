#!/usr/bin/env python3

import fileinput

measures = [int(line.strip()) for line in fileinput.input("input.txt")]

solution = [measures[i] < measures[i + 1] for i in range(len(measures) - 1)].count(True)
print(solution)
