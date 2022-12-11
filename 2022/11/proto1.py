from typing import Callable


class Monkey(object):
    def __init__(
        self,
        items: list[int],
        op: Callable[[int], int],
        test: int,
        succ: int,
        fail: int,
    ):
        self.items = items
        self.op = op
        self.test = test
        self.succ = succ
        self.fail = fail
        self.inspected = 0

    def compute_turn(self) -> list[tuple[int, int]]:
        ret_list = []
        while len(self.items) != 0:
            self.inspected += 1
            item = self.items.pop(0)
            item = self.op(item)
            item = item // 3
            if item % self.test == 0:
                ret_list.append((self.succ, item))
            else:
                ret_list.append((self.fail, item))
        return ret_list


def rounds(monkeys: list[Monkey], n: int) -> list[Monkey]:
    for i in range(n):
        for idx, monkey in enumerate(monkeys):
            throws = monkey.compute_turn()
            for target, item in throws:
                monkeys[target].items.append(item)
    return monkeys


def parse_op(fn_string: str) -> Callable[[int], int]:
    op_str = fn_string.split("=")[1].strip()

    if "+" in op_str:
        if op_str.endswith("old"):
            return lambda x: x + x
        else:
            v = int(op_str.split("+")[1])
            return lambda x: x + v
    else:
        if op_str.endswith("old"):
            return lambda x: x * x
        else:
            v = int(op_str.split("*")[1])
            return lambda x: x * v


def main() -> None:
    monkeys: list[Monkey] = []

    with open("input.txt", "r") as f:
        in_data = f.read()

        blocks = in_data.split("\n\n")
        for block in blocks:
            lines = block.split("\n")
            items = [int(v) for v in lines[1].split(":")[1].split(",")]
            op = parse_op(lines[2].split(":")[1])
            test_n = int(lines[3].split("divisible by")[1])
            target_succ = int(lines[4].split("monkey")[1])
            target_fail = int(lines[5].split("monkey")[1])

            monkeys.append(Monkey(items, op, test_n, target_succ, target_fail))

    monkeys = rounds(monkeys, 20)
    inspected = [monkey.inspected for monkey in monkeys]
    inspected.sort()
    a, b = inspected[-2:]
    print(f"{a} {b} {a * b}")


if __name__ == "__main__":
    main()
