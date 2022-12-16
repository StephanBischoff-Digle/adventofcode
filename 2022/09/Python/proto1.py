directions = {
    "L": (-1, 0),
    "R": (1, 0),
    "D": (0, -1),
    "U": (0, 1)
}


class Tail(object):
    def __init__(self):
        self.x = 0
        self.y = 0

        self.visited = set()

    def is_touching_head(self, head_x: int, head_y: int) -> bool:
        x_touching = head_x in [self.x - 1, self.x, self.x + 1]
        y_touching = head_y in [self.y - 1, self.y, self.y + 1]
        return x_touching and y_touching

    def update(self, head_x: int, head_y: int) -> None:
        if self.is_touching_head(head_x, head_y):
            return

        dx = head_x - self.x
        if dx < -1:
            dx = -1
        if dx > 1:
            dx = 1

        dy = head_y - self.y
        if dy < -1:
            dy = -1
        if dy > 1:
            dy = 1

        self.x += dx
        self.y += dy
        self.visited.add((self.x, self.y))


class Head(object):
    def __init__(self):
        self.x = 0
        self.y = 0
        self.tail = Tail()

    def update(self, command: str):
        direction, steps = command.split(" ")
        dx, dy = directions[direction]
        for _ in range(int(steps)):
            self.x += dx
            self.y += dy
            self.tail.update(self.x, self.y)


with open("input.txt", "r") as file:
    lines = [line.strip() for line in file.readlines()]

head = Head()
for line in lines:
    head.update(line)

print(len(head.tail.visited))
