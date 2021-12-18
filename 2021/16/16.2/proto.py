#!/usr/bin/env python3

import fileinput
from dataclasses import dataclass
import logging as log
from typing import Union
from functools import reduce

log.basicConfig(format="\033[1;34m%(levelname)s\033[0m:%(message)s", level=log.INFO)


hex_to_bin = {
    "0": "0000",
    "1": "0001",
    "2": "0010",
    "3": "0011",
    "4": "0100",
    "5": "0101",
    "6": "0110",
    "7": "0111",
    "8": "1000",
    "9": "1001",
    "a": "1010",
    "b": "1011",
    "c": "1100",
    "d": "1101",
    "e": "1110",
    "f": "1111",
}

Packet = Union["Literal", "Operator"]


@dataclass
class Header(object):
    version: int = 0
    type_id: int = 0
    size: int = 0


@dataclass
class Literal(object):
    header: Header
    value: int


@dataclass
class Operator(object):
    header: Header
    ltid: int
    subpackages: list[Packet]


def packet_parser(binary: str) -> Packet:
    log.debug("Parsing Packet")
    log.debug(f"binary: {binary}")

    header = Header()
    header.version = int(binary[:3], 2)
    header.type_id = int(binary[3:6], 2)

    log.debug(f"Header: {header}")
    if header.type_id == 4:
        start = 6
        group_size = 5
        literal = ""
        current = binary[start : start + group_size]
        while current[0] == "1":
            log.debug(f"literal block: {current}")
            literal += current[1:]
            start += group_size
            current = binary[start : start + group_size]
        log.debug(f"last literal block: {current}")
        literal += current[1:]

        header.size = start + group_size
        packet = Literal(header, int(literal, 2))
        log.debug(f"\033[0;31m{packet}\033[0m")
        return packet
    else:
        ltid = int(binary[6])
        packet = Operator(header, ltid, [])
        if ltid == 0:
            start = 7
            end = start + 15
            n_bits = int(binary[start:end], 2)
            log.debug(f"LTID-0-length: {n_bits}")
            sub_start = end
            pack_end = end + n_bits
            header.size = pack_end
            while sub_start < pack_end:
                sub = packet_parser(binary[sub_start:pack_end])
                packet.subpackages.append(sub)
                sub_start += sub.header.size
        else:
            start = 7
            end = start + 11
            n_packs = int(binary[start:end], 2)
            sub_start = end
            for _ in range(n_packs):
                sub = packet_parser(binary[sub_start:])
                sub_start += sub.header.size
                packet.subpackages.append(sub)
            packet.header.size = sub_start

        log.debug(f"\033[0;36m{packet}\033[0m")
        return packet


def eval_pack(p: Packet) -> int:
    if p.header.type_id == 4:
        assert isinstance(p, Literal)
        return p.value

    assert isinstance(p, Operator)
    if p.header.type_id == 0:
        return reduce(lambda a, b: a + b, [eval_pack(sub) for sub in p.subpackages])
    if p.header.type_id == 1:
        return reduce(lambda a, b: a * b, [eval_pack(sub) for sub in p.subpackages])
    if p.header.type_id == 2:
        return reduce(lambda a, b: min(a, b), [eval_pack(sub) for sub in p.subpackages])
    if p.header.type_id == 3:
        return reduce(lambda a, b: max(a, b), [eval_pack(sub) for sub in p.subpackages])

    assert len(p.subpackages) == 2
    s1 = eval_pack(p.subpackages[0])
    s2 = eval_pack(p.subpackages[1])
    if p.header.type_id == 5:
        return 1 if s1 > s2 else 0
    if p.header.type_id == 6:
        return 1 if s1 < s2 else 0
    if p.header.type_id == 7:
        return 1 if s1 == s2 else 0

    return 0


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


def main() -> None:
    in_lst = read_input()
    for line in in_lst:
        code = line.lower()
        log.info(f"code: {code}")
        binary = "".join([hex_to_bin[c] for c in code])
        p = packet_parser(binary)

        log.info(f"\033[1;31mSolution {eval_pack(p)}\033[0m")


if __name__ == "__main__":
    main()
