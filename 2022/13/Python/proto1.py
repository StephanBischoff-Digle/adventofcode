import sys
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


def read_pairs(filename: str) -> list[list[str, str]]:
    lst = []
    with open(filename, "r") as f:
        pairs = []
        for line in f.readlines():
            if line.strip() == "":
                lst.append(pairs)
                pairs = []
            else:
                pairs.append(line.strip())
        lst.append(pairs)
    return lst


def length_condition(first, second, i: int, prefix) -> Optional[bool]:
    print(f"{prefix}{i=}\n{prefix}first=  {first}\n{prefix}second= {second}")
    if i == len(first):
        if i < len(second):
            print(f"{prefix}\033[34;3mfirst end -> True\033[0m")
            return True
        elif i == len(second):
            print(f"{prefix}\033[34;3minconclusive -> None\033[0m")
            return None
    if i == len(second) and i < len(first):
        print(f"{prefix}\033[34;3msecond end -> False\033[0m")
        return False
    return None


def check_loop(first, second, prefix="") -> Optional[bool]:
    i = -1
    prefix = prefix + " "
    while True:
        i += 1
        length_cond = length_condition(first, second, i, prefix)
        if length_cond is not None:
            return length_cond
        if i == len(first) and i == len(second):
            print(f"{prefix}\033[34;3minconclusive -> None\033[0m")
            return None

        a = first[i]
        b = second[i]

        if a is None and b is not None:
            return True
        elif a is not None and b is None:
            return False

        print(f"{prefix}\033[34;3m{i=}: {a=}  {b=}\033[0m")
        if type(a) == int and type(b) == int:
            if a > b:
                print(f"{prefix}\033[34;3m{a} > {b} -> False\033[0m")
                return False
            if a < b:
                print(f"{prefix}\033[34;3m{a} < {b} -> True\033[0m")
                return True
            print(f"{prefix}\033[34;3m{a} = {b}\033[0m")
            continue
        if type(a) == list and type(b) == list:
            c = check_pair(a, b, prefix)
            if c is not None:
                return c
            continue
        if type(a) == int and type(b) == list:
            c = check_pair([a], b, prefix)
            if c is not None:
                return c
            continue
        if type(a) == list and type(b) == int:
            c = check_pair(a, [b], prefix)
            if c is not None:
                return c
            continue
        print("{prefix}\033[32;3mNone\033[0m")
        return None


def check_pair(first, second, prefix="") -> bool:
    print(
        f"{prefix}\033[32;3mfirst=  {first}\n{prefix}second= {second}\033[0m")
    return check_loop(first, second, prefix)


def main(interactive=False) -> None:
    lst = read_pairs("input.txt")

    for pair in lst:
        print(pair)

    acc = 0
    for idx, (a, b) in enumerate(lst):
        print(idx+1)
        print(a)
        print(b)

        pa = parse_list(a)
        print(f"\033[31m{pa=}\033[0m")

        pb = parse_list(b)
        print(f"\033[31m{pb=}\033[0m")
        check_result = check_pair(pa, pb)
        color = "\033[32m" if check_result else "\033[33m"
        print(f"{color}{check_result}\033[0m")
        print()
        if check_result:
            acc += idx+1
            print(f"{acc=} | {idx+1}")
        if interactive:
            input()

    print(f"\n\n\n{acc=}")


if __name__ == "__main__":
    if len(sys.argv) > 1:
        if sys.argv[1] == "-i":
            main(True)
            exit(0)
    main()
