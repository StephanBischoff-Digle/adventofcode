#!/usr/bin/env python3

import fileinput
import logging as log
from typing import Optional
from itertools import permutations
import json

log.basicConfig(format="\033[1;34m%(levelname)s\033[0m: %(message)s", level=log.WARN)


class BNode(object):

    h_code = "\033[1;32m"
    o_code = "\033[0m"

    def __init__(self, l, r) -> None:
        self.l = l
        self.r = r
        self.marked = False
        self.l_marked = False
        self.r_marked = False

    def unmark(self) -> None:
        self.marked = False
        self.l_marked = False
        self.r_marked = False
        if isinstance(self.l, BNode):
            self.l.unmark()
        if isinstance(self.r, BNode):
            self.r.unmark()

    def magnitude(self) -> int:
        l = 0
        if isinstance(self.l, int):
            l = self.l
        else:
            l = self.l.magnitude()

        r = 0
        if isinstance(self.r, int):
            r = self.r
        else:
            r = self.r.magnitude()

        return 3 * l + 2 * r

    def __repr__(self) -> str:
        repr_str = ""
        if self.marked:
            repr_str += BNode.h_code
        repr_str += "["

        if self.l_marked:
            repr_str += BNode.h_code
        repr_str += f"{self.l}"
        if self.l_marked and not self.marked:
            repr_str += BNode.o_code
        repr_str += ","

        if self.r_marked:
            repr_str += BNode.h_code
        repr_str += f"{self.r}"
        if self.r_marked and not self.marked:
            repr_str += BNode.o_code

        repr_str += "]"
        if self.marked:
            repr_str += BNode.o_code
        return repr_str

    def __add__(self, other: "BNode") -> "BNode":
        return BNode(self, other)


def parse(a: str) -> BNode:
    depth = 0
    l_split = ""
    r_split = ""
    for idx, c in enumerate(a):
        if c == "[":
            depth += 1
            continue
        if c == "]":
            depth -= 1
            continue
        if c == "," and depth == 1:
            l_split = a[1:idx]
            r_split = a[idx + 1 : -1]
            break
    l = ""
    r = ""

    try:
        l = int(l_split)
    except ValueError:
        l = parse(l_split)

    try:
        r = int(r_split)
    except ValueError:
        r = parse(r_split)

    return BNode(l, r)


def check_explode(a: BNode, depth: int = 0) -> bool:
    if depth >= 4 and isinstance(a.l, int) and isinstance(a.r, int):
        return True

    check_l = False
    if isinstance(a.l, BNode):
        check_l = check_explode(a.l, depth + 1)
        if check_l:
            return True

    check_r = False
    if isinstance(a.r, BNode):
        check_r = check_explode(a.r, depth + 1)
        if check_r:
            return True

    return False


def check_split(a: BNode) -> bool:
    if isinstance(a.l, int):
        if a.l >= 10:
            return True
    else:
        ls = check_split(a.l)
        if ls:
            return True

    if isinstance(a.r, int):
        if a.r >= 10:
            return True
    else:
        rs = check_split(a.r)
        if rs:
            return True

    return False


def split(a: BNode) -> tuple[BNode, bool]:
    if isinstance(a.l, int):
        if a.l > 9:
            vl = a.l // 2
            vr = a.l - vl
            a.l = BNode(vl, vr)
            a.l.marked = True
            return (a, True)

    if isinstance(a.l, BNode):
        _, has_split = split(a.l)
        if has_split:
            return (a, True)

    if isinstance(a.r, int):
        if a.r > 9:
            vl = a.r // 2
            vr = a.r - vl
            a.r = BNode(vl, vr)
            a.r.marked = True
            return (a, True)

    if isinstance(a.r, BNode):
        _, has_split = split(a.r)
        if has_split:
            return (a, True)

    return (a, False)


def explode(a: BNode) -> BNode:
    def get_by_hash(node: BNode, node_hash: str) -> BNode:
        log.debug(f"\tsearching for {node_hash}")
        current = node
        for c in node_hash:
            current = current.l if c == "l" else current.r
        return current

    def get_next_l(node: BNode, node_hash: str) -> Optional[tuple[int, str]]:
        cs = node_hash
        cs = cs.rstrip("l")

        log.debug(f"\tget_next_l cs pre strip: {cs}")
        if len(cs) == 0:
            log.debug("\t\tcs null")
            return None
        cs = cs[:-1] + "l"

        log.debug(f"\tget_next_l cs post strip: {cs}")
        current = get_by_hash(node, cs)
        while not isinstance(current, int):
            log.debug(f"{current} not int")
            cs += "r"

            log.debug(f"\tget_next_l cs: {cs}")
            current = get_by_hash(node, cs)
        return (current, cs)

    def get_next_r(node: BNode, node_hash: str) -> Optional[tuple[int, str]]:
        cs = node_hash
        cs = cs.rstrip("r")

        log.debug(f"\tget_next_r cs pre strip: {cs}")
        if len(cs) == 0:
            log.debug("\t\tcs null")
            return None
        cs = cs[:-1] + "r"

        log.debug(f"\tget_next_r cs post strip: {cs}")
        current = get_by_hash(node, cs)
        while not isinstance(current, int):
            log.debug(f"{current} not int")
            cs += "l"

            log.debug(f"\tget_next_r cs: {cs}")
            current = get_by_hash(node, cs)
        return (current, cs)

    def find_ex_pair(
        node: BNode, node_hash: str, depth: int = 0
    ) -> Optional[tuple[int, int, str]]:
        if depth >= 4:
            log.debug(f"\t depht: {depth}, hash: {node_hash} → {node}")
            if isinstance(node.l, int) and isinstance(node.r, int):
                log.debug(f"\t\tfound explosion pair")
                return (node.l, node.r, node_hash)
        if isinstance(node.l, BNode):
            l = find_ex_pair(node.l, node_hash + "l", depth + 1)
            if l:
                return l
        if isinstance(node.r, BNode):
            r = find_ex_pair(node.r, node_hash + "r", depth + 1)
            if r:
                return r

    log.debug("search for explosion pair")
    pair = find_ex_pair(a, "")
    if pair:
        pl, pr, ploc = pair

        log.debug("search for l")
        l = get_next_l(a, ploc)
        if l:
            data, loc = l

            log.debug(f"found l {data} at {loc}")
            data = data + pl
            log.debug("find l parent node")
            parent_node = get_by_hash(a, loc[:-1])
            if loc[-1] == "l":
                parent_node.l = data
                parent_node.l_marked = True
            else:
                parent_node.r = data
                parent_node.r_marked = True

        log.debug("search for r")
        r = get_next_r(a, ploc)
        if r:
            data, loc = r
            log.debug(f"found r {data} at {loc}")
            data = data + pr
            log.debug("find r parent node")
            parent_node = get_by_hash(a, loc[:-1])
            if loc[-1] == "l":
                parent_node.l = data
                parent_node.l_marked = True
            else:
                parent_node.r = data
                parent_node.r_marked = True

        log.debug("find pair root node")
        parent_node = get_by_hash(a, ploc[:-1])
        if ploc[-1] == "l":
            parent_node.l = 0
            parent_node.l_marked = True
        else:
            parent_node.r = 0
            parent_node.r_marked = True

    return a


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


def reduce(node: BNode, add_nr: int = 0) -> BNode:

    log.info(f"\033[1;31m{node}\033[0m")
    exp = check_explode(node)
    spl = check_split(node)

    current = node
    i = 0
    last = str(current)
    while exp or spl:
        i += 1
        if exp:
            current = explode(current)
            log.info(f"{add_nr}.{i}: after explode:   {current}")
        if spl and not exp:
            current, _ = split(current)
            log.info(f"{add_nr}.{i}: after split:     {current}")

        node.unmark()
        exp = check_explode(node)
        spl = check_split(node)
        if last == str(current):
            log.warning("No change!")
            raise ValueError

    return current


def main() -> None:
    in_lst = read_input()

    perms = [(parse(p[0]), parse(p[1])) for p in permutations(in_lst, 2)]

    max_mag = 0
    n_max = len(perms)
    print(f"checking {len(in_lst)} input lines ({n_max} permutations)")
    for i, (a, b) in enumerate(perms):
        max_mag = max(reduce(a + b).magnitude(), max_mag)
        print(
            "\t\t{:3d}%: {:3d}/{:3d} → {}".format(
                (100 * i + 1) // n_max, i + 1, n_max, max_mag
            ),
            end="\r",
        )

    print()
    print(f"Maximum Magnitude: {max_mag}")


if __name__ == "__main__":
    main()
