from functools import cmp_to_key
from typing import Optional


def parse_list(s: str) -> list:
    lst_str = s.strip()[1:-1]
    if lst_str == "":
        return []
    splits = []
    current_str = ""
    sub_depth = 0
    for c in lst_str:
        if c == "[":
            sub_depth += 1
            current_str += c
        elif c == "]":
            sub_depth -= 1
            current_str += c
        elif c == "," and sub_depth == 0:
            if current_str.isnumeric():
                splits.append(int(current_str))
            else:
                splits.append(parse_list(current_str))
            current_str = ""
            continue
        else:
            current_str += c

    if current_str.isnumeric():
        splits.append(int(current_str))
    else:
        splits.append(parse_list(current_str))
    return splits


def read(filename: str) -> list[list[str, str]]:
    lst = []
    with open(filename, "r") as f:
        for line in f.readlines():
            stripped = line.strip()
            if stripped != "":
                lst.append(stripped)
    return lst


def length_condition(first, second, i: int) -> Optional[bool]:
    if i == len(first):
        if i < len(second):
            return True
        elif i == len(second):
            return None
    if i == len(second) and i < len(first):
        return False
    return None


def check_loop(first, second) -> Optional[bool]:
    i = -1
    while True:
        i += 1
        length_cond = length_condition(first, second, i)
        if length_cond is not None:
            return length_cond
        if i == len(first) and i == len(second):
            return None

        a = first[i]
        b = second[i]

        if type(a) == int and type(b) == int:
            if a > b:
                return False
            if a < b:
                return True
            continue
        if type(a) == list and type(b) == list:
            c = check_pair(a, b)
            if c is not None:
                return c
            continue
        if type(a) == int and type(b) == list:
            c = check_pair([a], b)
            if c is not None:
                return c
            continue
        if type(a) == list and type(b) == int:
            c = check_pair(a, [b])
            if c is not None:
                return c
            continue
        return None


def check_pair(first, second) -> bool:
    return check_loop(first, second)


def pair_comparator(first, second) -> int:
    if check_pair(first, second):
        return -1
    return 1


def main() -> None:
    lst = list(map(lambda line: parse_list(line), read("input.txt")))
    lst.append([[2]])
    lst.append([[6]])

    lst.sort(key=cmp_to_key(pair_comparator))

    idx_2 = lst.index([[2]]) + 1
    idx_6 = lst.index([[6]]) + 1

    print(idx_2 * idx_6)


if __name__ == "__main__":
    main()
