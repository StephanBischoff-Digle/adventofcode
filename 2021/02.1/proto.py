#!/usr/bin/env python3
import fileinput


class Pos(object):
    def __init__(self) -> None:
        self.horizontal = 0
        self.depth = 0

        self.cmd_dispatch = {
            "forward": lambda val: self._forward(val),
            "down": lambda val: self._down(val),
            "up": lambda val: self._up(val),
        }

    def _forward(self, x: int) -> None:
        self.horizontal += x

    def _down(self, x: int) -> None:
        self.depth += x

    def _up(self, x: int) -> None:
        self.depth -= x

    def _strip_cmd(self, cmd: str) -> tuple[str, int]:
        split = cmd.split(" ")
        assert len(split) == 2
        word = split[0]
        val = int(split[1])
        return (word, val)

    def interpret_cmd(self, cmd: str) -> None:
        (word, val) = self._strip_cmd(cmd)
        self.cmd_dispatch[word](val)


def main() -> None:
    pos = Pos()
    for line in fileinput.input("input.txt"):
        line = str(line.strip())
        pos.interpret_cmd(line)

    print(pos.horizontal * pos.depth)


if __name__ == "__main__":
    main()
