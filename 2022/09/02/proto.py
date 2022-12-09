directions = {"L": (-1, 0), "R": (1, 0), "D": (0, -1), "U": (0, 1)}


class Tail(object):
    def __init__(self):
        self.x = 0
        self.y = 0

        self.visited = set()
        self.visited.add((0, 0))

    def is_touching_head(self, head_x: int, head_y: int) -> bool:
        x_touching = head_x in [self.x - 1, self.x, self.x + 1]
        y_touching = head_y in [self.y - 1, self.y, self.y + 1]
        return x_touching and y_touching

    def update(self, head_x: int, head_y: int) -> None:
        if self.is_touching_head(head_x, head_y):
            return

        dx = max(-1, min(1, head_x - self.x))
        dy = max(-1, min(1, head_y - self.y))

        self.x += dx
        self.y += dy
        self.visited.add((self.x, self.y))


class Head(object):
    def __init__(self):
        self.x = 0
        self.y = 0
        self.tails = [Tail() for _ in range(9)]

        self.x_min = 0
        self.x_max = 0
        self.y_min = 0
        self.y_max = 0

    def update(self, command: str):
        direction, steps = command.split(" ")
        dx, dy = directions[direction]
        for _ in range(int(steps)):
            self.x += dx
            self.y += dy
            self.tails[0].update(self.x, self.y)

            for i in range(1, len(self.tails)):
                self.tails[i].update(self.tails[i - 1].x, self.tails[i - 1].y)

    def print(self):
        xs = [self.x] + [self.tails[i].x for i in range(len(self.tails))]
        ys = [self.y] + [self.tails[i].y for i in range(len(self.tails))]

        self.x_min = min(xs + [self.x_min])
        self.y_min = min(ys + [self.y_min])
        self.x_max = max(xs + [self.x_max])
        self.y_max = max(ys + [self.y_max])

        for y in reversed(range(head.y_min - 3, head.y_max + 4)):
            for x in range(head.x_min - 3, head.x_max + 4):
                covered = []
                if x == self.x and y == self.y:
                    covered.append(0)
                for i, tail in enumerate(self.tails):
                    if x == tail.x and y == tail.y:
                        covered.append(i+1)
                if len(covered) > 0:
                    print(f"{covered[0]}", end="")
                else:
                    print(".", end="")
            print()


with open("input.txt", "r") as file:
    lines = [line.strip() for line in file.readlines()]

head = Head()
for line in lines:
    # print("==============================")
    # print(line)
    head.update(line)
    # head.print()

print(len(head.tails[-1].visited))


for y in reversed(range(head.y_min - 3, head.y_max + 4)):
    for x in range(head.x_min - 3, head.x_max + 4):
        if (x, y) in head.tails[-1].visited:
            print("#", end="")
        else:
            print(".", end="")
    print()
