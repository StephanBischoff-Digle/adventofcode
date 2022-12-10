import sys

FileSystem = list["INode"]


class INode(object):
    def __init__(
            self, idx: int, parent: int, size: int = 0, is_dir: bool = False
    ) -> None:
        self.size = size
        self.content = {}
        if is_dir:
            self.content["."] = idx
            self.content[".."] = parent

    def is_dir(self) -> bool:
        return self.size == 0

    def __str__(self) -> str:
        ret = ""
        if self.size == 0:
            for key, val in self.content.items():
                ret += f"{key}\t{val}\n"
        else:
            ret = f"file: {self.size}\n"
        return ret


def terminal(fs: FileSystem) -> None:
    cmd = "ls"
    pos = 0
    path = "/"
    while cmd not in ["exit", "q", "quit"]:
        cmd = input(f"\033[33m{path}\033[0m » ")
        if cmd in ["exit", "q", "quit"]:
            print("k, thx, bye")
            break

        if cmd.startswith("ls"):
            name = cmd[3:]
            cpos = pos
            if name:
                if name in fs[pos].content:
                    cpos = fs[pos].content[name]
                else:
                    print("\033[31mNo such directory\033[0m")
                    continue
            kv = list(fs[cpos].content.items())
            kv.sort(key=lambda x: x[0])
            dirs = list(filter(lambda x: fs[x[1]].is_dir(), kv))
            files = list(filter(lambda x: not fs[x[1]].is_dir(), kv))
            for key, val in dirs:
                print(f"          \033[32;1m{key}\033[0m")
            for key, val in files:
                print(f"{fs[val].size:9} {key}")
            continue

        if cmd.startswith("cd"):
            name = cmd[3:]
            if not name:
                pos = 0
                path = "/"
                continue

            if name not in fs[pos].content:
                print("\033[31mNo such directory\033[0m")
                continue

            if not fs[fs[pos].content[name]].is_dir():
                print(f"\033[31m{name} is a file\033[0m")
                continue

            pos = fs[pos].content[name]

            if pos == 0:
                path = "/"
                continue
            path = ""
            cpos = pos
            while cpos != 0:
                tpos = fs[cpos].content[".."]
                for k, v in fs[tpos].content.items():
                    if v == cpos:
                        path = path + f"/{k}"
                        break
                cpos = tpos
            continue

        if cmd == "pwd":
            if pos == 0:
                print("/")
                continue
            path = ""
            cpos = pos
            while cpos != 0:
                tpos = fs[cpos].content[".."]
                for k, v in fs[tpos].content.items():
                    if v == cpos:
                        path = path + f"/{k}"
                        break
                cpos = tpos
            print(path)
            continue

        if cmd not in ["h", "help"]:
            print(f"\033[31m{cmd} is not a valid command\033[0m")
        print("The following \033[34mcommands\033[0m are supported:")
        print(
            "\033[34mcd\033[0m [\033[35mname\033[0m]        -- changes directory into \033[35mname\033[0m, if no name is given: change to \033[35m/\033[0m")
        print("\033[34mls\033[0m [\033[35mname\033[0m]        -- prints the content of \033[35mname\033[0m, if no name is give: print the content of the current directory")
        print(
            "\033[34mpwd\033[0m              -- prints the path to the current directory")
        print(
            "\033[34mq\033[0m, \033[34mquit\033[0m, \033[34mexit\033[0m    -- terminates the shell")
        print(
            "\033[34mh\033[0m, \033[34mhelp\033[0m          -- print this help message")


def solve(fs: FileSystem) -> int:
    sizes: dict[int, int] = {}
    more_passes = 1

    while more_passes > 0:
        more_passes = 0
        for idx, node in enumerate(fs):
            if node.is_dir() and idx not in sizes:
                size = 0
                escape = False
                for k, c in node.content.items():
                    if k in [".", ".."]:
                        continue
                    if fs[c].is_dir() and c not in sizes:
                        escape = True
                        more_passes += 1
                        break
                    if fs[c].is_dir() and c in sizes:
                        size += sizes[c]
                    if not fs[c].is_dir():
                        size += fs[c].size
                if escape:
                    continue
                else:
                    sizes[idx] = size
    n = sum(filter(lambda v: v <= 100000, sizes.values()))
    return n


with open("input.txt", "r") as f:
    lines = f.readlines()

fs = [INode(0, 0, is_dir=True)]


pos = 0
line_idx = 1
while line_idx < len(lines):
    line = lines[line_idx].strip()
    if line.startswith("$ ls"):
        line_idx += 1
        ls_l = lines[line_idx].strip()
        while not ls_l.startswith("$"):
            if ls_l.startswith("dir"):
                _, name = ls_l.split(" ")
                my_idx = len(fs)
                fs.append(INode(my_idx, pos, is_dir=True))
                fs[pos].content[name] = my_idx
            else:
                size, name = ls_l.split(" ")
                my_idx = len(fs)
                fs.append(INode(my_idx, pos, size=int(size)))
                fs[pos].content[name] = my_idx

            line_idx += 1
            if line_idx >= len(lines):
                break
            ls_l = lines[line_idx].strip()
    else:
        name = line[5:]
        pos = fs[pos].content[name]
        line_idx += 1


if len(sys.argv) > 1:
    if sys.argv[1] == "-i":
        terminal(fs)
else:
    print(solve(fs))
