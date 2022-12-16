reg = 1
cycle = 0

cursor_pos = 0

res = ""

with open("input.txt", "r") as f:
    for line in f.readlines():
        if line.startswith("noop"):
            cycle += 1
            cursor_pos = cycle % 40
            if cursor_pos in [reg, reg+1, reg+2]:
                res += "#"
            else:
                res += "."
            if cycle % 40 == 0:
                res += "\n"

        if line.startswith("addx"):
            _, v = line.split()

            cycle += 1

            cursor_pos = cycle % 40
            if cursor_pos in [reg, reg+1, reg+2]:
                res += "#"
            else:
                res += "."
            if cycle % 40 == 0:
                res += "\n"

            cycle += 1
            cursor_pos = cycle % 40
            if cursor_pos in [reg, reg+1, reg+2]:
                res += "#"
            else:
                res += "."
            if cycle % 40 == 0:
                res += "\n"

            reg += int(v)
print(res)
