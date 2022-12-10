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

    total_space = 70000000
    needed_space = 30000000
    used_space = sizes[0]
    remaining = total_space - used_space
    delta = needed_space - remaining

    print(f"total:  {total_space:15}")
    print(f"needed: {needed_space:15}")
    print(f"used:   {used_space:15}")
    print(f"rem:    {remaining:15}")
    print(f"delta:  {delta:15}")

    s_list = list(filter(lambda x: x > delta, sizes.values()))
    sorted(s_list)

    return s_list[0]


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


print(solve(fs))
