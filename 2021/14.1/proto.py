#!/usr/bin/env python3

import fileinput
from collections import Counter

Rules = dict[str, str]


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


def parse_rules(lines: list[str]) -> Rules:
    r = {}
    for line in lines:
        split = line.split(" -> ")
        r[split[0]] = "{}{}".format(split[1], split[0][1])
    return r


def apply_rules(template: str, rules: Rules) -> str:
    new_template = template[0]
    for i in range(len(template) - 1):
        new_template += rules[template[i : i + 2]]
    return new_template


def main() -> None:
    in_lst = read_input()
    template = in_lst[0]
    rules = parse_rules(in_lst[2:])

    for _ in range(10):
        template = apply_rules(template, rules)

    c = dict(Counter(template))
    max_c = max(c.values())
    min_c = min(c.values())
    print(f"{max_c - min_c}")


if __name__ == "__main__":
    main()
