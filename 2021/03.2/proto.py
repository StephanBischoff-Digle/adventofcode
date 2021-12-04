#!/usr/bin/env python3

import fileinput


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


def gamma_eps(lst: list[str]) -> tuple[int, int]:
    acc = {}

    # count bits
    for line in lst:
        for (idx, c) in enumerate(line):
            acc[idx] = acc.get(idx, 0) + int(c)

    l = len(lst)
    gamma = 0
    mask = 0
    for (k, v) in acc.items():
        # decide if the bit is relevant
        d = 0 if 2 * v < l else 1
        gamma += d << (len(acc) - k - 1)
        mask += 1 << (len(acc) - k - 1)

    epsilon = gamma ^ mask
    return (gamma, epsilon)


def filter(mask_gen, lst: list[str]) -> str:
    f_lst = lst
    l = len(lst[0]) - 1
    idx = 0
    while len(f_lst) > 1:
        mask = mask_gen(f_lst)
        d = 0 if (mask & 1 << l) == 0 else 1
        f_lst = [w for w in f_lst if int(w[idx]) == d]
        idx += 1
        l -= 1

    assert len(f_lst) == 1
    return f_lst[0]


def filter_lst(lst: list[str]) -> tuple[str, str]:

    g_gen = lambda x: gamma_eps(x)[0]
    e_gen = lambda x: gamma_eps(x)[1]
    return (filter(g_gen, lst), filter(e_gen, lst))


def main() -> None:
    in_lst = read_input()
    (g, e) = filter_lst(in_lst)

    print(f"solution: {int(g, 2) * int(e, 2)}")


if __name__ == "__main__":
    main()
