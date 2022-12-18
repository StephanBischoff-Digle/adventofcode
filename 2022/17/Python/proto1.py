import time


def parse(filename: str) -> str:
    with open(filename, "r") as f:
        line = f.readline().strip()
        return line


Field = list[int]
Rock = list[int]
rocks = [
    [0b0011110],
    [0b0001000, 0b0011100, 0b0001000],
    [0b0000100, 0b0000100, 0b0011100],
    [0b0010000, 0b0010000, 0b0010000, 0b0010000],
    [0b0011000, 0b0011000],
]

state_print_map = [[" ", "\033[33m@\033[0m"],
                   ["\033[34m#\033[0m", "\033[31m×\033[0m"]]


def left_collision(rock: Rock, top: int, field: Field) -> bool:
    for y in range(len(rock)):
        r_l = rock[y] << 1
        left_mask = 128 + field[top - y]
        if r_l & left_mask > 0:
            return True


def right_collision(rock: Rock, top: int, field: Field) -> bool:
    for y in range(len(rock)):
        r_l = rock[y] >> 1
        if rock[y] & 1 > 0 or r_l & field[top - y] > 0:
            return True


def bottom_collision(rock: Rock, top: int, field: Field) -> bool:
    if top - len(rock) < 0:
        return True

    for y in range(len(rock)):
        if field[top - y - 1] & rock[y] > 0:
            return True
    return False


def print_state(field: Field, rock: Rock, top: int, wind: str, commands: str, cmd_idx: int, n_rocks: int):
    n_lines = 40
    print(f"\r\u001b[{n_lines+5}A")
    l_wind = "»" if wind == ">" else " "
    r_wind = "«" if wind == "<" else " "
    highest_rock_index = highest_rock(field)
    c_per_line = max(30, len(commands)//(n_lines-1))
    print(f"rock: {n_rocks}")
    for idx, line in enumerate(field[::-1]):
        cmd_str = ""
        if idx * c_per_line < len(commands):
            for c in range(idx * c_per_line, min(len(commands), (idx+1) * c_per_line)):
                if c == cmd_idx:
                    cmd_str = cmd_str + f"\033[46m{commands[c]}\033[0m"
                else:
                    cmd_str = cmd_str + commands[c]

        if idx == n_lines:
            break
        idx = len(field) - 1 - idx
        rock_index = top - idx
        r_str = "0000000"
        if rock_index >= 0 and rock_index < len(rock):
            r_str = "{0:07b}".format(rock[rock_index])
        s = "{0:07b}".format(line)
        l_str = "".join([state_print_map[int(a)][int(b)]
                        for a, b in zip(s, r_str)])
        i_str = "     "

        l_delim = "│"
        r_delim = "│"
        if idx % 10 == 0 and idx != highest_rock_index:
            i_str = f"{idx:5}"
            l_str = l_str.replace(" ", "\033[36m┄\033[0m")
            r_delim = "\033[36m┾\033[0m"
            l_delim = "\033[36m┽\033[0m"
        if idx == highest_rock_index:
            i_str = f"\033[31m{idx:5}\033[0m"
            l_str = l_str.replace(" ", "\033[31m┄\033[0m")
            r_delim = "\033[31m┾\033[0m"
            l_delim = "\033[31m┽\033[0m"

        print(f"{i_str} {l_wind} {l_delim}{l_str}{r_delim} {r_wind}       {cmd_str}")
        # print(f"{i_str} {l_wind} |{l_str}| {r_wind}")
    if n_lines < len(field):
        print(f"\033[35m{len(field)-n_lines:5}   ┽┄┄┄┄┄┄┄┾\033[0m")
    else:
        print("     0  └───────┘")


def spaw_rock(i: int) -> Rock:
    return rocks[i % 5].copy()


def highest_rock(field: Field) -> int:
    for idx, line in enumerate(field):
        if line == 0:
            return idx - 1
    return 0


def required_field_hight(field: Field, rock: Rock) -> int:
    return highest_rock(field) + len(rock) + 4


def main():
    commands = parse("test.txt")
    rock_id = 1
    rock = spaw_rock(0)

    field = [0] * required_field_hight([0], rock)
    rt = len(field) - 1
    t = 0

    r_rocks = 0
    use_r_rocks = False
    t_rocks = 2022
    n_rocks = 0
    while n_rocks < t_rocks:
        # push rock
        if commands[t] == "<" and not left_collision(rock, rt, field):
            for i in range(len(rock)):
                rock[i] = rock[i] << 1
        elif commands[t] == ">" and not right_collision(rock, rt, field):
            for i in range(len(rock)):
                rock[i] = rock[i] >> 1
        print_state(field, rock, rt, commands[t], commands, t, n_rocks)

        if bottom_collision(rock, rt, field):
            for i in range(len(rock)):
                field[rt - i] += rock[i]

            n_rocks += 1
            if n_rocks == r_rocks and use_r_rocks:
                use_r_rocks = False
            if n_rocks == t_rocks:
                break
            rock = spaw_rock(rock_id)

            required_hight = required_field_hight(field, rock)
            additional = required_hight - len(field)
            if additional > 0:
                field += [0]*additional
            if additional < 0:
                field = field[:additional]

            rt = len(field) - 1
            rock_id += 1
        else:
            rt -= 1

        print_state(field, rock, rt, "", commands, t, n_rocks)
        t = (t+1) % len(commands)
        if not use_r_rocks:
            in_str = input()
            if in_str != "":
                use_r_rocks = True
                r_rocks = int(in_str)

    print_state(field, rock, rt, "", commands, t, n_rocks)
    print(highest_rock(field)+1)


if __name__ == "__main__":
    main()
