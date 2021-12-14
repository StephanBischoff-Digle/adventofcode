#!/usr/bin/env python3


import fileinput


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


class Field(object):
    def __init__(self, v: int) -> None:
        self._v = v
        self._x = False

    def mark(self, v: int) -> None:
        if not self._x and self._v == v:
            self._x = True

    def is_marked(self) -> bool:
        return self._x

    def value(self) -> int:
        return self._v


class Board(object):
    def __init__(self, lst: list[str]) -> None:
        self._field = [
            [Field(int(v.strip())) for v in line.split(" ") if v.strip()]
            for line in lst
        ]

    def call_nr(self, v: int) -> None:
        [[f.mark(v) for f in line] for line in self._field]

    def check_bingo(self) -> bool:
        return self._h() or self._v()

    def _h(self) -> bool:
        return any(
            [all(map(lambda elem: elem.is_marked(), line)) for line in self._field]
        )

    def _v(self) -> bool:
        for (idx, _) in enumerate(self._field):
            if all([line[idx].is_marked() for line in self._field]):
                return True
        return False

    def get_unmarked(self) -> list[int]:
        return [x.value() for line in self._field for x in line if not x.is_marked()]


def main() -> None:
    in_lst = read_input()
    seq = [int(v) for v in in_lst[0].split(",")]

    # Construction
    boards = []
    collector = []
    for line in in_lst[2:]:
        if line == "":
            boards.append(Board(collector))
            collector = []
        else:
            collector.append(line)

    # Game
    last = None
    for n in seq:
        [board.call_nr(n) for board in boards]
        if len(boards) > 1:
            boards = list(filter(lambda board: not board.check_bingo(), boards))
        else:
            if boards[0].check_bingo():
                last = n
                break

    solution = last * sum(boards[0].get_unmarked())
    print(solution)


if __name__ == "__main__":
    main()
