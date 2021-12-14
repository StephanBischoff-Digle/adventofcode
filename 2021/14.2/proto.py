#!/usr/bin/env python3

import fileinput
from collections import defaultdict, Counter

Rules = dict[str, tuple[str, str]]
Freq = dict[str, int]


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


def parse_rules(lines: list[str]) -> Rules:
    """
    The line format is `AB -> C`. The generated Rule has the format `AB: (AC, CB)`
    such that we can track what new pairs a rule generates.
    """
    r = {}
    for line in lines:
        split = line.split(" -> ")
        p1 = "{}{}".format(split[0][0], split[1])
        p2 = "{}{}".format(split[1], split[0][1])
        r[split[0]] = (p1, p2)
    return r


def update_frequencies(pairs: Freq, elements: Freq, rules: Rules) -> tuple[Freq, Freq]:
    """
    Updates the pair and element frequencies. A pair splits into two new pairs and
    generates a new element.
    """
    new_freq = defaultdict(lambda: 0)
    new_elem = defaultdict(lambda: 0)
    new_elem.update(elements)
    for (k, v) in rules.items():
        (p1, p2) = v
        new_freq[p1] += pairs[k]
        new_freq[p2] += pairs[k]
        gen_elem = p1[1]
        new_elem[gen_elem] += pairs[k]

    return (new_freq, new_elem)


def initial_frequencies(template: str, rules: Rules) -> Freq:
    """
    Generates the initial pair frequencies from the template and the rule set.
    """
    frequencies = defaultdict(lambda: 0)
    for i in range(len(template) - 1):
        key = template[i : i + 2]
        frequencies[key] += 1
    return frequencies


def main() -> None:
    in_lst = read_input()
    template = in_lst[0]
    rules = parse_rules(in_lst[2:])

    frequencies = (initial_frequencies(template, rules), dict(Counter(template)))

    # conveniece lambda
    step = lambda fs: update_frequencies(fs[0], fs[1], rules)

    for _ in range(40):
        frequencies = step(frequencies)

    elements = dict(frequencies[1])
    solution = max(elements.values()) - min(elements.values())

    print(solution)


if __name__ == "__main__":
    main()
