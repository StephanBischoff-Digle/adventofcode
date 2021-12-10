#!/usr/bin/env python3

import fileinput
import operator
from functools import reduce
from typing import Optional


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


mapping = {
    #   a  b  c  d  e  f  g
    0: [1, 1, 1, 0, 1, 1, 1],
    1: [0, 0, 1, 0, 0, 1, 0],
    2: [1, 0, 1, 1, 1, 0, 1],
    3: [1, 0, 1, 1, 0, 1, 1],
    4: [0, 1, 1, 1, 0, 1, 0],
    5: [1, 1, 0, 1, 0, 1, 1],
    6: [1, 1, 0, 1, 1, 1, 1],
    7: [1, 0, 1, 0, 0, 1, 0],
    8: [1, 1, 1, 1, 1, 1, 1],
    9: [1, 1, 1, 1, 0, 1, 1],
}


ctoidx: dict[str, int] = {"a": 0, "b": 1, "c": 2, "d": 3, "e": 4, "f": 5, "g": 6}


def print_matrix(A: list[list[int]]) -> None:
    for row in A:
        print(f"{row}")


def m_x_v(M: list[list[int]], v: list[int]) -> list[int]:
    return [reduce(operator.add, map(operator.mul, r, v)) for r in M]


def str_to_vec(s: str) -> list[int]:
    init = [0] * 7
    for c in s:
        init[ctoidx[c]] = 1
    return init


def transform(s: str, A: list[list[int]]) -> Optional[int]:
    s_v = str_to_vec(s)
    s_v_transformed = m_x_v(A, s_v)
    for (k, v) in mapping.items():
        if v == s_v_transformed:
            return k
    return None


def deduce(line: str) -> int:
    parts = line.split(" | ")
    pattern = parts[0].split(" ")
    digits = parts[1].split(" ")

    K = [
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
    ]

    l2 = list(filter(lambda x: len(x) == 2, pattern))[0]
    l3 = list(filter(lambda x: len(x) == 3, pattern))[0]
    l4 = list(filter(lambda x: len(x) == 4, pattern))[0]

    # infere 'c' and 'f' from l2 (1)
    s_cf = set()
    for c in l2:
        s_cf.add(c)

    # infere 'a' from l3 (7)
    a = ""
    for c in l3:
        if not c in l2:
            a = c
            break

    # infere 'b' and 'd' from l4 (4)
    s_bd = set()
    for c in l4:
        if not c in l3:
            s_bd.add(c)

    # find 9 by combining l4 with 'a' segment of length 6
    # we find 'g', the segment that is contained
    # we find 'e', the segment that is not contained
    l6s = list(filter(lambda x: len(x) == 6, pattern))
    s_9 = {k for k in l4}.union(s_cf)

    l9 = list(filter(lambda word: all([k in word for k in s_9]), l6s))[0]
    g = ""
    for c in l9:
        if not c in l4 and not c in l3:
            g = c
            break
    e = list(filter(lambda x: x not in l9, "abcdefg"))[0]
    s_9.add(g)

    # determine c and f finally using 6
    s_6 = s_bd.union({e, g, a})
    f = ""
    l6 = list(filter(lambda word: all([k in word for k in s_6]), l6s))[0]
    for c in l6:
        if not c in s_6:
            f = c
            break
    c = list(filter(lambda x: x not in l6, "abcdefg"))[0]

    # determine b and d finally using 0
    s_0 = set([a, c, e, f, g])
    l0 = list(filter(lambda word: all([k in word for k in s_0]), l6s))[0]
    b = ""
    for k in l0:
        if not k in s_0:
            b = k
            break
    d = list(filter(lambda x: x not in l0, "abcdefg"))[0]

    K[ctoidx["a"]][ctoidx[a]] = 1
    K[ctoidx["b"]][ctoidx[b]] = 1
    K[ctoidx["c"]][ctoidx[c]] = 1
    K[ctoidx["d"]][ctoidx[d]] = 1
    K[ctoidx["e"]][ctoidx[e]] = 1
    K[ctoidx["f"]][ctoidx[f]] = 1
    K[ctoidx["g"]][ctoidx[g]] = 1

    ds = [transform(d, K) for d in digits]

    val = 0
    for (i, d) in enumerate(reversed(ds)):
        val += d * 10 ** i

    return val


def main() -> None:
    in_lst = read_input()
    vals = [deduce(line) for line in in_lst]
    print(sum(vals))


if __name__ == "__main__":
    main()
